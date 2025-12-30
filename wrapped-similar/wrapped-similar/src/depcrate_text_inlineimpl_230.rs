// Generated macro for impl_230 (impl)
macro_rules! Depcrate_text_inlineimpl_230 {
() => {
// Module: crate::text::inline
// Provides: {"impl_230"}
// Dependencies: {}
impl < T : DiffableStr + ? Sized > fmt :: Display for InlineChange < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (emphasized , value) in self . iter_strings_lossy () { let marker = match (emphasized , self . tag) { (false , _) | (true , ChangeTag :: Equal) => "" , (true , ChangeTag :: Delete) => "-" , (true , ChangeTag :: Insert) => "+" , } ; write ! (f , "{}{}{}" , marker , value , marker) ? ; } if self . missing_newline () { writeln ! (f) ? ; } Ok (()) } }
};
}
