// Generated macro for impl_112 (impl)
macro_rules! Depcrate_testutilimpl_112 {
() => {
// Module: crate::testutil
// Provides: {"impl_112"}
// Dependencies: {}
impl Sink for KitchenSink { type Error = io :: Error ; fn matched (& mut self , _searcher : & Searcher , mat : & SinkMatch < '_ > ,) -> Result < bool , io :: Error > { assert ! (! mat . bytes () . is_empty ()) ; assert ! (mat . lines () . count () >= 1) ; let mut line_number = mat . line_number () ; let mut byte_offset = mat . absolute_byte_offset () ; for line in mat . lines () { if let Some (ref mut n) = line_number { write ! (self . 0 , "{}:" , n) ? ; * n += 1 ; } write ! (self . 0 , "{}:" , byte_offset) ? ; byte_offset += line . len () as u64 ; self . 0 . write_all (line) ? ; } Ok (true) } fn context (& mut self , _searcher : & Searcher , context : & SinkContext < '_ > ,) -> Result < bool , io :: Error > { assert ! (! context . bytes () . is_empty ()) ; assert ! (context . lines () . count () == 1) ; if let Some (line_number) = context . line_number () { write ! (self . 0 , "{}-" , line_number) ? ; } write ! (self . 0 , "{}-" , context . absolute_byte_offset) ? ; self . 0 . write_all (context . bytes ()) ? ; Ok (true) } fn context_break (& mut self , _searcher : & Searcher ,) -> Result < bool , io :: Error > { self . 0 . write_all (b"--\n") ? ; Ok (true) } fn finish (& mut self , _searcher : & Searcher , sink_finish : & SinkFinish ,) -> Result < () , io :: Error > { writeln ! (self . 0 , "") ? ; writeln ! (self . 0 , "byte count:{}" , sink_finish . byte_count ()) ? ; if let Some (offset) = sink_finish . binary_byte_offset () { writeln ! (self . 0 , "binary offset:{}" , offset) ? ; } Ok (()) } }
};
}
