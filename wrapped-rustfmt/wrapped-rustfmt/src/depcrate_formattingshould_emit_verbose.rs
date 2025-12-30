// Generated macro for should_emit_verbose (function)
macro_rules! Depcrate_formattingshould_emit_verbose {
() => {
// Module: crate::formatting
// Provides: {"should_emit_verbose"}
// Dependencies: {}
fn should_emit_verbose < F > (forbid_verbose_output : bool , config : & Config , f : F) where F : Fn () , { if config . verbose () == Verbosity :: Verbose && ! forbid_verbose_output { f () ; } }
};
}
