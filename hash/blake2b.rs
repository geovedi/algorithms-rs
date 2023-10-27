use std::convert::TryInto;

const BLOCK_SIZE: usize = 128;
const WORD_SIZE: usize = 8;
const DIGEST_SIZE: usize = 64;
const KEY_SIZE: usize = 64;
const SALT_SIZE: usize = 16;
const PERSONAL_SIZE: usize = 16;

const MASK8BITS: u8 = 0xff;
const MASK16BITS: u16 = 0xffff;
const MASK32BITS: u32 = 0xffffffff;
const MASK48BITS: u64 = 0xffffffffffff;
const MASK64BITS: u64 = 0xffffffffffffffff;

const ROT1: u32 = 32;
const ROT2: u32 = 24;
const ROT3: u32 = 16;
const ROT4: u32 = 63;

struct BLAKE2b {
    h: [u64; 8],
    t: [u64; 2],
    f: [u64; 2],
    buflen: usize,
    buf: [u8; BLOCK_SIZE],
    key: Option<[u8; KEY_SIZE]>,
    finalized: bool,
}

// Constants for the mixing function 'G' in 12 rounds
const SIGMA: [[usize; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    // Add more SIGMA rounds as needed
];

impl BLAKE2b {
    fn new(digest_size: usize, key: Option<[u8; KEY_SIZE]>) -> Result<Self, &'static str> {
        if digest_size <= 0 || digest_size > DIGEST_SIZE {
            return Err("Invalid digest size");
        }

        let mut blake2b = BLAKE2b {
            h: [
                0x6a09e667f3bcc908,
                0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b,
                0xa54ff53a5f1d36f1,
                0x510e527fade682d1,
                0x9b05688c2b3e6c1f,
                0x1f83d9abfb41bd6b,
                0x5be0cd19137e2179,
            ],
            t: [0, 0],
            f: [0, 0],
            buflen: 0,
            buf: [0; BLOCK_SIZE],
            key,
            finalized: false,
        };

        blake2b.t[0] = digest_size as u64;
        if let Some(key) = &blake2b.key {
            blake2b.t[1] = KEY_SIZE as u64;
            blake2b.buflen = BLOCK_SIZE;
            blake2b.update(key);
        }

        Ok(blake2b)
    }

    fn init(&mut self, salt: [u8; SALT_SIZE], personal: [u8; PERSONAL_SIZE]) {
        if self.finalized {
            return;
        }

        let mut param_bytes = [0u8; 64];

        param_bytes[0] = self.h.len() as u8;
        param_bytes[1] = DIGEST_SIZE as u8;
        param_bytes[2] = 1; // fanout
        param_bytes[3] = 1; // depth
        param_bytes[4..8].copy_from_slice(&(BLOCK_SIZE as u32).to_le_bytes());
        param_bytes[8..12].copy_from_slice(&(0u32).to_le_bytes()); // node offset
        param_bytes[12] = 0; // node depth
        param_bytes[13] = 0; // inner length
        param_bytes[14..30].copy_from_slice(&salt);
        param_bytes[30..46].copy_from_slice(&personal);

        let mut params = [0u64; 8];
        for i in 0..8 {
            params[i] = u64::from_le_bytes(param_bytes[i * 8..(i + 1) * 8].try_into().unwrap());
        }

        for i in 0..8 {
            self.h[i] ^= params[i];
        }
    }

    fn update(&mut self, data: &[u8]) {
        if self.finalized {
            return;
        }

        let mut data_ptr = 0;
        let data_len = data.len();
        let mut offset = 0;

        while offset < data_len {
            if self.buflen == BLOCK_SIZE {
                self.t[0] += BLOCK_SIZE as u64;
                if self.t[0] < BLOCK_SIZE as u64 {
                    self.t[1] += 1;
                }
                self.compress();
                self.buflen = 0;
            }

            self.buf[self.buflen] = data[offset];
            self.buflen += 1;
            data_ptr += 1;
            offset += 1;
        }
    }

    fn finalize(&mut self) -> [u8; DIGEST_SIZE] {
        if self.finalized {
            return Default::default();
        }

        self.t[0] += self.buflen as u64;
        if self.t[0] < self.buflen as u64 {
            self.t[1] += 1;
        }

        self.f[0] = MASK64BITS;
        self.f[1] = MASK64BITS;

        for i in 0..self.buflen {
            self.buf[i] ^= MASK8BITS;
        }

        self.compress();

        let mut result = [0u8; DIGEST_SIZE];
        for i in 0..DIGEST_SIZE {
            result[i] = (self.h[i / WORD_SIZE] >> (8 * (i % WORD_SIZE))) as u8;
        }

        self.finalized = true;
        result
    }

    fn hexdigest(&mut self) -> String {
        let digest = self.finalize();
        let hex_chars: Vec<String> = digest.iter().map(|&byte| format!("{:02x}", byte)).collect();
        hex_chars.join("")
    }

    fn compress(&mut self) {
        let mut v: [u64; 16] = [0; 16];
        let mut m: [u64; 16] = [0; 16];

        // Convert the input block (self.buf) into an array of u64 words (little-endian)
        for i in 0..16 {
            m[i] = u64::from_le_bytes([
                self.buf[i * 8],
                self.buf[i * 8 + 1],
                self.buf[i * 8 + 2],
                self.buf[i * 8 + 3],
                self.buf[i * 8 + 4],
                self.buf[i * 8 + 5],
                self.buf[i * 8 + 6],
                self.buf[i * 8 + 7],
            ]);
        }

        // Initialize the working vector 'v' with the current state 'h'
        v.copy_from_slice(&self.h);

        // Mixing function 'G' applied in 12 rounds
        for round in 0..12 {
            let sigma = SIGMA[round];
            for i in 0..16 {
                // Mixing function 'G'
                v = self.g(v, i, sigma[i % 10], m);
            }
        }

        // Update the state 'h' with the result
        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }

    // Mixing function 'G'
    fn g(&self, mut v: [u64; 16], i: usize, sigma: [usize; 16], m: [u64; 16]) -> [u64; 16] {
        let (mut a, mut b, mut c, mut d) = (v[sigma[0]], v[sigma[1]], v[sigma[2]], v[sigma[3]]);

        a = a
            .wrapping_add(b)
            .wrapping_add(m[sigma[0]])
            .wrapping_add(self.c[2 * i]);
        d ^= a;
        d = d.rotate_right(ROT1);

        c = c.wrapping_add(d);
        b ^= c;
        b = b.rotate_right(ROT2);

        a = a
            .wrapping_add(b)
            .wrapping_add(m[sigma[1]])
            .wrapping_add(self.c[2 * i + 1]);
        d ^= a;
        d = d.rotate_right(ROT3);

        c = c.wrapping_add(d);
        b ^= c;
        b = b.rotate_right(ROT4);

        v[sigma[0]] = a;
        v[sigma[1]] = b;
        v[sigma[2]] = c;
        v[sigma[3]] = d;

        v
    }
}

fn main() {
    let data = b"Hello, BLAKE2b!";
    let key: Option<[u8; KEY_SIZE]> = None;
    let salt: [u8; SALT_SIZE] = [0u8; SALT_SIZE];
    let personal: [u8; PERSONAL_SIZE] = [0u8; PERSONAL_SIZE];

    let mut blake2b = BLAKE2b::new(DIGEST_SIZE, key).unwrap();
    blake2b.init(salt, personal);
    blake2b.update(data);

    let hex_digest = blake2b.hexdigest();

    println!("Hex Digest: {}", hex_digest);
}
