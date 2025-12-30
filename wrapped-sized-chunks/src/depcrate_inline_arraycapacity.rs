// Generated macro for capacity (function)
macro_rules! Depcrate_inline_arraycapacity {
() => {
// Module: crate::inline_array
// Provides: {"capacity"}
// Dependencies: {}
const fn capacity (host_size : usize , header_size : usize , element_size : usize , element_align : usize , container_align : usize ,) -> usize { if element_size == 0 { usize :: MAX } else if element_align <= container_align && host_size > header_size { (host_size - header_size) / element_size } else { 0 } }
};
}
