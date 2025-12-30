// Generated macro for impl_781 (impl)
macro_rules! Depcrate_mm_mmapimpl_781 {
() => {
// Module: crate::mm::mmap
// Provides: {"impl_781"}
// Dependencies: {}
impl MapFlags { # [doc = " Create `MAP_HUGETLB` with provided size of huge page."] # [doc = ""] # [doc = " Under the hood it computes"] # [doc = " `MAP_HUGETLB | (huge_page_size_log2 << MAP_HUGE_SHIFT)`."] # [doc = " `huge_page_size_log2` denotes logarithm of huge page size to use and"] # [doc = " should be between 16 and 63 (inclusive)."] # [doc = ""] # [doc = " ```"] # [doc = " use rustix::mm::MapFlags;"] # [doc = ""] # [doc = " let f = MapFlags::hugetlb_with_size_log2(30).unwrap();"] # [doc = " assert_eq!(f, MapFlags::HUGETLB | MapFlags::HUGE_1GB);"] # [doc = " ```"] # [cfg (linux_kernel)] pub const fn hugetlb_with_size_log2 (huge_page_size_log2 : u32) -> Option < Self > { use crate :: backend :: c ; if 16 <= huge_page_size_log2 && huge_page_size_log2 <= 63 { let bits = bitcast ! (c :: MAP_HUGETLB) | (huge_page_size_log2 << c :: MAP_HUGE_SHIFT) ; Self :: from_bits (bits) } else { None } } }
};
}
