macro_rules! deps {
    () => {
        Crc32Fold!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl Crc32Fold { pub const fn new () -> Self { Self :: new_with_initial (CRC32_INITIAL_VALUE) } pub const fn new_with_initial (initial : u32) -> Self { Self { # [cfg (target_arch = "x86_64")] fold : pclmulqdq :: Accumulator :: new () , value : initial , } } pub fn fold (& mut self , src : & [u8] , _start : u32) { # [cfg (target_arch = "x86_64")] if crate :: cpu_features :: is_enabled_pclmulqdq () { return unsafe { self . fold . fold (src , _start) } ; } # [cfg (target_arch = "aarch64")] if crate :: cpu_features :: is_enabled_crc () { self . value = unsafe { self :: acle :: crc32_acle_aarch64 (self . value , src) } ; return ; } self . value = braid :: crc32_braid :: < 5 > (self . value , src) ; } pub fn fold_copy (& mut self , dst : & mut [u8] , src : & [u8]) { # [cfg (target_arch = "x86_64")] if crate :: cpu_features :: is_enabled_pclmulqdq () { return unsafe { self . fold . fold_copy (dst , src) } ; } self . fold (src , 0) ; dst [.. src . len ()] . copy_from_slice (src) ; } pub fn finish (self) -> u32 { # [cfg (target_arch = "x86_64")] if crate :: cpu_features :: is_enabled_pclmulqdq () { return unsafe { self . fold . finish () } ; } self . value } }
    };
}

impl_107!();