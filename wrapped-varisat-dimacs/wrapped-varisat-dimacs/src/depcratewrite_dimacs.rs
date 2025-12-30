// Generated macro for write_dimacs (function)
macro_rules! Depcratewrite_dimacs {
() => {
// Module: crate
// Provides: {"write_dimacs"}
// Dependencies: {}
# [doc = " Write a formula as DIMACS CNF."] # [doc = ""] # [doc = " Use [`write_dimacs_header`] and [`write_dimacs_clauses`] to implement incremental writing."] pub fn write_dimacs (target : & mut impl io :: Write , formula : & CnfFormula) -> io :: Result < () > { write_dimacs_header (& mut * target , DimacsHeader { var_count : formula . var_count () , clause_count : formula . len () , } ,) ? ; write_dimacs_clauses (& mut * target , formula . iter ()) }
};
}
