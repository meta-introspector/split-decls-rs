// Generated macro for impl_100 (impl)
macro_rules! Depcrate_writerimpl_100 {
() => {
// Module: crate::writer
// Provides: {"impl_100"}
// Dependencies: {}
impl < W : io :: Write > LineWriter < W > { fn new (wtr : W) -> LineWriter < W > { LineWriter { wtr , line : String :: new () , columns : 79 , indent : "  " . to_string () , } } fn write_str (& mut self , s : & str) -> io :: Result < () > { if self . line . len () + s . len () > self . columns { self . flush_line () ? ; } if self . line . is_empty () { self . line . push_str (& self . indent) ; } self . line . push_str (s) ; Ok (()) } fn indent (& mut self , s : & str) { self . indent = s . to_string () ; } fn flush_line (& mut self) -> io :: Result < () > { if self . line . is_empty () { return Ok (()) ; } self . wtr . write_all (self . line . trim_end () . as_bytes ()) ? ; self . wtr . write_all (b"\n") ? ; self . line . clear () ; Ok (()) } }
};
}
