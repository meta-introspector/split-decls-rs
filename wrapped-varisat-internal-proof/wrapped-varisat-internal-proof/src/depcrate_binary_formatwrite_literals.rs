// Generated macro for write_literals (function)
macro_rules! Depcrate_binary_formatwrite_literals {
() => {
// Module: crate::binary_format
// Provides: {"write_literals"}
// Dependencies: {}
# [doc = " Writes a slice of literals for a varisat proof"] fn write_literals (target : & mut impl Write , literals : & [Lit]) -> io :: Result < () > { write_u64 (& mut * target , literals . len () as u64) ? ; for & lit in literals { write_u64 (& mut * target , lit . code () as u64) ? ; } Ok (()) }
};
}
