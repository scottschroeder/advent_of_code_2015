#[derive(Debug)]
pub struct PermuteDistribute {
    buf: Vec<i64>,
    cursor: Vec<(usize, i64)>,
}

impl PermuteDistribute {
    pub fn new(len: usize, total: i64) -> PermuteDistribute {
        assert!(len > 0);
        PermuteDistribute {
            buf: vec![0; len],
            cursor: vec![(0, total)],
        }
    }

    pub fn pop(&mut self) {
        if let Some((start_idx, total)) = self.cursor.pop() {
            self.buf[start_idx] = total;
            for idx in (start_idx..self.buf.len()).skip(1) {
                self.buf[idx] = 0
            }
        }
    }
}

// impl Iterator for PermuteDistribute {
//     type Item = &[i64];

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
