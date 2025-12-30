// Generated macro for impl_1254 (impl)
macro_rules! Depcrate_lru_disk_cacheimpl_1254 {
() => {
// Module: crate::lru_disk_cache
// Provides: {"impl_1254"}
// Dependencies: {}
# [doc = " Given a tuple of (path, filesize), use the filesize for measurement."] impl < K > Meter < K , u64 > for FileSize { type Measure = usize ; fn measure < Q : ? Sized > (& self , _ : & Q , v : & u64) -> usize where K : Borrow < Q > , { * v as usize } }
};
}
