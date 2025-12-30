// Generated macro for u8to64_le (macro)
macro_rules! Depcrate_testsu8to64_le {
() => {
// Module: crate::tests
// Provides: {"u8to64_le"}
// Dependencies: {}
macro_rules ! u8to64_le { ($ buf : expr , $ i : expr) => { $ buf [0 + $ i] as u64 | ($ buf [1 + $ i] as u64) << 8 | ($ buf [2 + $ i] as u64) << 16 | ($ buf [3 + $ i] as u64) << 24 | ($ buf [4 + $ i] as u64) << 32 | ($ buf [5 + $ i] as u64) << 40 | ($ buf [6 + $ i] as u64) << 48 | ($ buf [7 + $ i] as u64) << 56 } ; ($ buf : expr , $ i : expr , $ len : expr) => { { let mut t = 0 ; let mut out = 0 ; while t < $ len { out |= ($ buf [t + $ i] as u64) << t * 8 ; t += 1 ; } out } } ; }
};
}
