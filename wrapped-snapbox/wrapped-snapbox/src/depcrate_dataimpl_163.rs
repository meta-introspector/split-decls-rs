// Generated macro for impl_163 (impl)
macro_rules! Depcrate_dataimpl_163 {
() => {
// Module: crate::data
// Provides: {"impl_163"}
// Dependencies: {}
impl std :: fmt :: Display for Data { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . inner { DataInner :: Error (data) => data . fmt (f) , DataInner :: Binary (data) => String :: from_utf8_lossy (data) . fmt (f) , DataInner :: Text (data) => data . fmt (f) , # [cfg (feature = "json")] DataInner :: Json (data) => serde_json :: to_string_pretty (data) . unwrap () . fmt (f) , # [cfg (feature = "json")] DataInner :: JsonLines (data) => { let array = data . as_array () . expect ("jsonlines is always an array") ; for value in array { writeln ! (f , "{}" , serde_json :: to_string (value) . unwrap ()) ? ; } Ok (()) } # [cfg (feature = "term-svg")] DataInner :: TermSvg (data) => data . fmt (f) , } } }
};
}
