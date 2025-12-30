// Generated macro for impl_419 (impl)
macro_rules! Depcrate_zipcryptoimpl_419 {
() => {
// Module: crate::zipcrypto
// Provides: {"impl_419"}
// Dependencies: {}
impl Debug for ZipCryptoKeys { # [allow (unreachable_code)] fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { # [cfg (not (any (test , fuzzing)))] { use std :: collections :: hash_map :: DefaultHasher ; use std :: hash :: Hasher ; let mut t = DefaultHasher :: new () ; self . hash (& mut t) ; f . write_fmt (format_args ! ("ZipCryptoKeys(hash {})" , t . finish ())) } # [cfg (any (test , fuzzing))] f . write_fmt (format_args ! ("ZipCryptoKeys::of({:#10x},{:#10x},{:#10x})" , self . key_0 , self . key_1 , self . key_2)) } }
};
}
