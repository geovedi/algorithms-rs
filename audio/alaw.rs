const A: f32 = 87.6;
const MAX_AMPLITUDE: f32 = 32767.0;

fn a_law_encode(sample: i16) -> u8 {
    let sample = sample as f32;
    let sign = if sample < 0.0 { 0x80 } else { 0x00 };
    let abs_sample = sample.abs();
    let compressed_sample = if abs_sample < 1.0 / A {
        A * abs_sample / (1.0 + A.log2())
    } else {
        1.0 + abs_sample.log2() / (1.0 + A.log2())
    };
    let quantized_sample = ((compressed_sample * 15.0).round() as u8) & 0x0F;
    sign | (quantized_sample << 4)
}

fn a_law_decode(encoded_sample: u8) -> i16 {
    let sign = if encoded_sample & 0x80 == 0x80 { -1 } else { 1 };
    let quantized_sample = ((encoded_sample & 0x0F) as f32) / 15.0;
    let abs_sample = if quantized_sample < 1.0 / A {
        quantized_sample * (1.0 + A.log2()) / A
    } else {
        (1.0 + A.log2()).exp2() * (quantized_sample - 1.0)
    };
    ((sign as f32 * abs_sample * MAX_AMPLITUDE).round() as i16).wrapping_mul(sign)
}

fn main() {
    test_a_law();
}

fn test_a_law() {
    let samples = vec![-32768, -16384, -8192, -4096, -2048, -1024, -512, -256, 0, 256, 512, 1024, 2048, 4096, 8192, 16384, 32767];
    for &sample in &samples {
        let encoded = a_law_encode(sample);
        let decoded = a_law_decode(encoded);
        println!("Original: {}, Encoded: {:02X}, Decoded: {}", sample, encoded, decoded);
        assert!((sample - decoded).abs() <= (MAX_AMPLITUDE / 15.0) as i16);
    }
}

