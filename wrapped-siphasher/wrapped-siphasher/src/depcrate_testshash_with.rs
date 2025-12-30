// Generated macro for hash_with (function)
macro_rules! Depcrate_testshash_with {
() => {
// Module: crate::tests
// Provides: {"hash_with"}
// Dependencies: {}
fn hash_with < H : Hasher , T : Hash > (mut st : H , x : & T) -> u64 { x . hash (& mut st) ; st . finish () }
};
}
