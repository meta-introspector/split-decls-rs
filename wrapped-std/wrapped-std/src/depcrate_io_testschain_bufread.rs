// Generated macro for chain_bufread (function)
macro_rules! Depcrate_io_testschain_bufread {
() => {
// Module: crate::io::tests
// Provides: {"chain_bufread"}
// Dependencies: {}
# [test] fn chain_bufread () { let testdata = b"ABCDEFGHIJKL" ; let chain1 = (& testdata [.. 3]) . chain (& testdata [3 .. 6]) . chain (& testdata [6 .. 9]) . chain (& testdata [9 ..]) ; let chain2 = (& testdata [.. 4]) . chain (& testdata [4 .. 8]) . chain (& testdata [8 ..]) ; cmp_bufread (chain1 , chain2 , & testdata [..]) ; }
};
}
