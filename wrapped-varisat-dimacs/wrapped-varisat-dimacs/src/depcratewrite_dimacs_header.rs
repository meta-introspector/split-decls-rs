// Generated macro for write_dimacs_header (function)
macro_rules! Depcratewrite_dimacs_header {
() => {
// Module: crate
// Provides: {"write_dimacs_header"}
// Dependencies: {}
# [doc = " Write a DIMACS CNF header."] # [doc = ""] # [doc = " Can be used with [`write_dimacs_clauses`] to implement incremental writing."] pub fn write_dimacs_header (target : & mut impl io :: Write , header : DimacsHeader) -> io :: Result < () > { writeln ! (target , "p cnf {var_count} {clause_count}" , var_count = header . var_count , clause_count = header . clause_count) }
};
}
