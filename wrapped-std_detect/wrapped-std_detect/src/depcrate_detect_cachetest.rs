// Generated macro for test (function)
macro_rules! Depcrate_detect_cachetest {
() => {
// Module: crate::detect::cache
// Provides: {"test"}
// Dependencies: {}
# [doc = " Tests the `bit` of the storage. If the storage has not been initialized,"] # [doc = " initializes it with the result of `os::detect_features()`."] # [doc = ""] # [doc = " On its first invocation, it detects the CPU features and caches them in the"] # [doc = " `CACHE` global variable as an `AtomicU64`."] # [doc = ""] # [doc = " It uses the `Feature` variant to index into this variable as a bitset. If"] # [doc = " the bit is set, the feature is enabled, and otherwise it is disabled."] # [doc = ""] # [doc = " If the feature `std_detect_env_override` is enabled looks for the env"] # [doc = " variable `RUST_STD_DETECT_UNSTABLE` and uses its content to disable"] # [doc = " Features that would had been otherwise detected."] # [inline] pub (crate) fn test (bit : u32) -> bool { let (relative_bit , idx) = if bit < Cache :: CAPACITY { (bit , 0) } else if bit < 2 * Cache :: CAPACITY { (bit - Cache :: CAPACITY , 1) } else { (bit - 2 * Cache :: CAPACITY , 2) } ; CACHE [idx] . test (relative_bit) . unwrap_or_else (| | detect_and_initialize () . test (bit)) }
};
}
