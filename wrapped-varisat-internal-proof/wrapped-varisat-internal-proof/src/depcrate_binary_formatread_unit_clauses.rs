// Generated macro for read_unit_clauses (function)
macro_rules! Depcrate_binary_formatread_unit_clauses {
() => {
// Module: crate::binary_format
// Provides: {"read_unit_clauses"}
// Dependencies: {}
# [doc = " Read a slice of unit clauses from a varisat proof"] fn read_unit_clauses (source : & mut impl BufRead , units : & mut Vec < (Lit , ClauseHash) > ,) -> Result < () , io :: Error > { units . clear () ; let len = read_u64 (& mut * source) ? as usize ; units . reserve (len) ; for _ in 0 .. len { let lit = Lit :: from_code (read_u64 (& mut * source) ? as usize) ; let hash = read_u64 (& mut * source) ? as ClauseHash ; units . push ((lit , hash)) ; } Ok (()) }
};
}
