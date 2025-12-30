// Generated macro for impl_83 (impl)
macro_rules! Depcrate_runnerimpl_83 {
() => {
// Module: crate::runner
// Provides: {"impl_83"}
// Dependencies: {}
impl std :: fmt :: Display for Stream { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let palette = snapbox :: report :: Palette :: color () ; match & self . status { StreamStatus :: Ok => { writeln ! (f , "{}:" , self . stream) ? ; writeln ! (f , "{}" , palette . info (& self . content)) ? ; } StreamStatus :: Failure (msg) => { writeln ! (f , "{} {}:" , self . stream , palette . error (format_args ! ("({msg})"))) ? ; writeln ! (f , "{}" , palette . info (& self . content)) ? ; } StreamStatus :: Expected (expected) => { snapbox :: report :: write_diff (f , expected , & self . content , Some (& self . stream) , Some (& self . stream) , palette ,) ? ; } } Ok (()) } }
};
}
