use rand::Rng;

struct Zobrist {
    ztable: Vec<Vec<u32>>,
}

impl Zobrist {
    fn new(rows: usize, columns: usize) -> Zobrist {
        let mut ztable = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..columns {
                row.push(rng.gen());
            }
            ztable.push(row);
        }

        Zobrist { ztable }
    }

    fn hash(&self, board: &[usize]) -> u32 {
        let mut hash = 0;
        for (i, &value) in board.iter().enumerate() {
            if i < self.ztable.len() && value < self.ztable[i].len() {
                hash ^= self.ztable[i][value];
            }
        }
        hash
    }
}

fn test_zobrist() {
    let zobrist = Zobrist::new(3, 3);
    let test_cases = vec![
        (vec![1, 2, 3], zobrist.hash(&[1, 2, 3])),
        (vec![2, 3, 1], zobrist.hash(&[2, 3, 1])),
        (vec![3, 1, 2], zobrist.hash(&[3, 1, 2])),
        (vec![1, 1, 1], zobrist.hash(&[1, 1, 1])),
        (vec![3, 3, 3], zobrist.hash(&[3, 3, 3])),
    ];

    for (i, (board, expected)) in test_cases.iter().enumerate() {
        let hash = zobrist.hash(&board);
        let passed = hash == *expected;
        println!("Test case {}: {}", i + 1, if passed { "PASSED" } else { "FAILED" });
    }
}

fn main() {
    test_zobrist();
}
