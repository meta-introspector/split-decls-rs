// Generated macro for read_literals (function)
macro_rules! Depcrate_binary_formatread_literals {
() => {
// Module: crate::binary_format
// Provides: {"read_literals"}
// Dependencies: {}
# [doc = " Read a slice of literals from a varisat proof"] fn read_literals (source : & mut impl BufRead , literals : & mut Vec < Lit >) -> Result < () , io :: Error > { literals . clear () ; let len = read_u64 (& mut * source) ? as usize ; literals . reserve (len) ; for _ in 0 .. len { literals . push (Lit :: from_code (read_u64 (& mut * source) ? as usize)) ; } Ok (()) }
};
}
