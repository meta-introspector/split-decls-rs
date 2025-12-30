// Generated macro for test_siphash128_1_3 (function)
macro_rules! Depcrate_tests128test_siphash128_1_3 {
() => {
// Module: crate::tests128
// Provides: {"test_siphash128_1_3"}
// Dependencies: {}
# [test] # [allow (unused_must_use)] fn test_siphash128_1_3 () { let vecs : [[u8 ; 16] ; 1] = [[231 , 126 , 188 , 178 , 39 , 136 , 165 , 190 , 253 , 98 , 219 , 106 , 221 , 48 , 48 , 1 ,]] ; let k0 = 0x_07_06_05_04_03_02_01_00 ; let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08 ; let mut buf = Vec :: new () ; let mut t = 0 ; let mut state_inc = SipHasher13 :: new_with_keys (k0 , k1) ; while t < 1 { let vec = vecs [t] ; let out = hash_with (SipHasher13 :: new_with_keys (k0 , k1) , & Bytes (& buf)) ; assert_eq ! (vec , out [..]) ; let full = hash_with (SipHasher13 :: new_with_keys (k0 , k1) , & Bytes (& buf)) ; let i = state_inc . finish128 () . as_bytes () ; assert_eq ! (full , i) ; assert_eq ! (full , vec) ; buf . push (t as u8) ; Hasher :: write (& mut state_inc , & [t as u8]) ; t += 1 ; } }
};
}
