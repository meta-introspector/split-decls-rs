// Generated macro for impl_342 (impl)
macro_rules! Depcrateimpl_342 {
() => {
// Module: crate
// Provides: {"impl_342"}
// Dependencies: {}
impl SessionGlobals { pub fn new (edition : Edition , extra_symbols : & [& 'static str] , sm_inputs : Option < SourceMapInputs > ,) -> SessionGlobals { SessionGlobals { symbol_interner : symbol :: Interner :: with_extra_symbols (extra_symbols) , span_interner : Lock :: new (span_encoding :: SpanInterner :: default ()) , metavar_spans : Default :: default () , hygiene_data : Lock :: new (hygiene :: HygieneData :: new (edition)) , source_map : sm_inputs . map (| inputs | Arc :: new (SourceMap :: with_inputs (inputs))) , } } }
};
}
