// Generated macro for write_unit_clauses (function)
macro_rules! Depcrate_binary_formatwrite_unit_clauses {
() => {
// Module: crate::binary_format
// Provides: {"write_unit_clauses"}
// Dependencies: {}
# [doc = " Writes a slice of unit clauses for a varisat proof"] fn write_unit_clauses (target : & mut impl Write , units : & [(Lit , ClauseHash)]) -> io :: Result < () > { write_u64 (& mut * target , units . len () as u64) ? ; for & (lit , hash) in units { write_u64 (& mut * target , lit . code () as u64) ? ; write_u64 (& mut * target , hash as u64) ? ; } Ok (()) }
};
}
