// Double the ciphertext (using addition)
// Example 1:
// ctxt += ctxt;
// [0] [1] [1] ... [1] [1] -> [0] [2] [2] ... [2] [2]
// Example 2:
// ctxt += &ctxt;
// [0] [1] [1] ... [1] [1] -> [0] [2] [2] ... [2] [2]

impl std::ops::AddAssign<Self> for Ctxt {
    fn add_assign(&mut self, other: Self) {
        // Implementation goes here
    }
}

// Square the ciphertext
// Example 1:
// ctxt *= &ctxt;
// [0] [1] [2] [3] [4] ... [nslots-1]
// -> [0] [1] [4] [9] [16] ... [(nslots-1)*(nslots-1)]

impl std::ops::MulAssign<&Self> for Ctxt {
    fn mul_assign(&mut self, other: &Self) {
        // Implementation goes here
    }
}

// Square the ciphertext
// Example 2:
// let ctxt2 = ctxt * ctxt;
// [0] [1] [2] [3] [4] ... [nslots-1]
// -> [0] [1] [4] [9] [16] ... [(nslots-1)*(nslots-1)]

impl std::ops::Mul<Self> for Ctxt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Implementation goes here
    }
}
