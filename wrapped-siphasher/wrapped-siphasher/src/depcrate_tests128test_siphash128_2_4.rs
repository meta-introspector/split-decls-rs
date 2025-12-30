// Generated macro for test_siphash128_2_4 (function)
macro_rules! Depcrate_tests128test_siphash128_2_4 {
() => {
// Module: crate::tests128
// Provides: {"test_siphash128_2_4"}
// Dependencies: {}
# [test] # [allow (unused_must_use)] fn test_siphash128_2_4 () { let vecs : [[u8 ; 16] ; 1] = [[163 , 129 , 127 , 4 , 186 , 37 , 168 , 230 , 109 , 246 , 114 , 20 , 199 , 85 , 2 , 147 ,]] ; let k0 = 0x_07_06_05_04_03_02_01_00 ; let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08 ; let mut buf = Vec :: new () ; let mut t = 0 ; let mut state_inc = SipHasher24 :: new_with_keys (k0 , k1) ; while t < 1 { let vec = vecs [t] ; let out = hash_with (SipHasher24 :: new_with_keys (k0 , k1) , & Bytes (& buf)) ; assert_eq ! (vec , out [..]) ; let full = hash_with (SipHasher24 :: new_with_keys (k0 , k1) , & Bytes (& buf)) ; let i = state_inc . finish128 () . as_bytes () ; assert_eq ! (full , i) ; assert_eq ! (full , vec) ; buf . push (t as u8) ; Hasher :: write (& mut state_inc , & [t as u8]) ; t += 1 ; } }
};
}
