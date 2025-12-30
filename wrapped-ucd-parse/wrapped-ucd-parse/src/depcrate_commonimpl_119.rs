// Generated macro for impl_119 (impl)
macro_rules! Depcrate_commonimpl_119 {
() => {
// Module: crate::common
// Provides: {"impl_119"}
// Dependencies: {}
impl < R : io :: Read , D : FromStr < Err = Error > > Iterator for UcdLineParser < R , D > { type Item = Result < D , Error > ; fn next (& mut self) -> Option < Result < D , Error > > { loop { self . line_number += 1 ; self . line . clear () ; let n = match self . rdr . read_line (& mut self . line) { Err (err) => { return Some (Err (Error { kind : ErrorKind :: Io (err) , line : None , path : self . path . clone () , })) } Ok (n) => n , } ; if n == 0 { return None ; } if ! self . line . starts_with ('#') && ! self . line . trim () . is_empty () { break ; } } let line_number = self . line_number ; Some (self . line . parse () . map_err (| mut err : Error | { err . line = Some (line_number) ; err })) } }
};
}
