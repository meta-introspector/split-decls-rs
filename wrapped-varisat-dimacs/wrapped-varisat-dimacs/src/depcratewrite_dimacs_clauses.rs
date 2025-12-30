// Generated macro for write_dimacs_clauses (function)
macro_rules! Depcratewrite_dimacs_clauses {
() => {
// Module: crate
// Provides: {"write_dimacs_clauses"}
// Dependencies: {}
# [doc = " Write an iterator of clauses as headerless DIMACS CNF."] # [doc = ""] # [doc = " Can be used with [`write_dimacs_header`] to implement incremental writing."] pub fn write_dimacs_clauses (target : & mut impl io :: Write , clauses : impl IntoIterator < Item = impl IntoIterator < Item = impl Borrow < Lit > > > ,) -> io :: Result < () > { for clause in clauses . into_iter () { for lit in clause . into_iter () { itoa :: write (& mut * target , lit . borrow () . to_dimacs ()) ? ; target . write_all (b" ") ? ; } target . write_all (b"0\n") ? ; } Ok (()) }
};
}
