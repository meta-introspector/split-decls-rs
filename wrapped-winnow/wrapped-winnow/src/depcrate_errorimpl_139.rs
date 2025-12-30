// Generated macro for impl_139 (impl)
macro_rules! Depcrate_errorimpl_139 {
() => {
// Module: crate::error
// Provides: {"impl_139"}
// Dependencies: {}
impl < I , E > core :: fmt :: Display for ParseError < I , E > where I : AsBStr , E : core :: fmt :: Display , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let input = self . input . as_bstr () ; let span_start = self . offset ; let span_end = span_start ; # [cfg (feature = "std")] if input . contains (& b'\n') { let (line_idx , col_idx) = translate_position (input , span_start) ; let line_num = line_idx + 1 ; let col_num = col_idx + 1 ; let gutter = line_num . to_string () . len () ; let content = input . split (| c | * c == b'\n') . nth (line_idx) . expect ("valid line number") ; writeln ! (f , "parse error at line {line_num}, column {col_num}") ? ; for _ in 0 .. gutter { write ! (f , " ") ? ; } writeln ! (f , " |") ? ; write ! (f , "{line_num} | ") ? ; writeln ! (f , "{}" , String :: from_utf8_lossy (content)) ? ; for _ in 0 .. gutter { write ! (f , " ") ? ; } write ! (f , " | ") ? ; for _ in 0 .. col_idx { write ! (f , " ") ? ; } write ! (f , "^") ? ; for _ in (span_start + 1) .. (span_end . min (span_start + content . len ())) { write ! (f , "^") ? ; } writeln ! (f) ? ; } else { let content = input ; writeln ! (f , "{}" , String :: from_utf8_lossy (content)) ? ; for _ in 0 .. span_start { write ! (f , " ") ? ; } write ! (f , "^") ? ; for _ in (span_start + 1) .. (span_end . min (span_start + content . len ())) { write ! (f , "^") ? ; } writeln ! (f) ? ; } write ! (f , "{}" , self . inner) ? ; Ok (()) } }
};
}
