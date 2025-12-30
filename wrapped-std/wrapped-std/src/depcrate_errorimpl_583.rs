// Generated macro for impl_583 (impl)
macro_rules! Depcrate_errorimpl_583 {
() => {
// Module: crate::error
// Provides: {"impl_583"}
// Dependencies: {}
impl < E > Report < E > where E : Error , { fn backtrace (& self) -> Option < & Backtrace > { let backtrace = request_ref (& self . error) ; let backtrace = backtrace . or_else (| | { self . error . source () . map (| source | source . sources () . find_map (| source | request_ref (source))) . flatten () }) ; backtrace } # [doc = " Format the report as a single line."] # [unstable (feature = "error_reporter" , issue = "90172")] fn fmt_singleline (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . error) ? ; let sources = self . error . source () . into_iter () . flat_map (< dyn Error > :: sources) ; for cause in sources { write ! (f , ": {cause}") ? ; } Ok (()) } # [doc = " Format the report as multiple lines, with each error cause on its own line."] # [unstable (feature = "error_reporter" , issue = "90172")] fn fmt_multiline (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let error = & self . error ; write ! (f , "{error}") ? ; if let Some (cause) = error . source () { write ! (f , "\n\nCaused by:") ? ; let multiple = cause . source () . is_some () ; for (ind , error) in cause . sources () . enumerate () { writeln ! (f) ? ; let mut indented = Indented { inner : f } ; if multiple { write ! (indented , "{ind: >4}: {error}") ? ; } else { write ! (indented , "      {error}") ? ; } } } if self . show_backtrace { if let Some (backtrace) = self . backtrace () { write ! (f , "\n\nStack backtrace:\n{}" , backtrace . to_string () . trim_end ()) ? ; } } Ok (()) } }
};
}
