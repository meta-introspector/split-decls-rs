// Generated macro for hash_with (function)
macro_rules! Depcrate_tests128hash_with {
() => {
// Module: crate::tests128
// Provides: {"hash_with"}
// Dependencies: {}
fn hash_with < H : Hasher + Hasher128 , T : Hash > (mut st : H , x : & T) -> [u8 ; 16] { x . hash (& mut st) ; st . finish128 () . as_bytes () }
};
}
