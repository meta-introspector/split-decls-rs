// Generated macro for DISASSEMBLY (static)
macro_rules! DepcrateDISASSEMBLY {
() => {
// Module: crate
// Provides: {"DISASSEMBLY"}
// Dependencies: {}
static DISASSEMBLY : LazyLock < HashSet < Function > > = LazyLock :: new (disassemble_myself) ;
};
}
