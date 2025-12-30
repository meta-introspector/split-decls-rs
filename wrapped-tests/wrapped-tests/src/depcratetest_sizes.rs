// Generated macro for test_sizes (function)
macro_rules! Depcratetest_sizes {
() => {
// Module: crate
// Provides: {"test_sizes"}
// Dependencies: {}
pub fn test_sizes < A : Allocator > (alloc : A) { test_allocate_layout (& alloc , Layout :: new :: < u8 > ()) ; test_allocate_layout (& alloc , Layout :: new :: < u16 > ()) ; test_allocate_layout (& alloc , Layout :: new :: < u32 > ()) ; test_allocate_layout (& alloc , Layout :: new :: < u64 > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 17] > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 67] > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 129] > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 654] > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 2345] > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 32578] > ()) ; test_allocate_layout (& alloc , Layout :: new :: < [u8 ; 8603461] > ()) ; }
};
}
