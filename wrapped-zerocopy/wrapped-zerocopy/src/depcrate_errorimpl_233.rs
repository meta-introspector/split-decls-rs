// Generated macro for impl_233 (impl)
macro_rules! Depcrate_errorimpl_233 {
() => {
// Module: crate::error
// Provides: {"impl_233"}
// Dependencies: {}
# [doc = " Produces a human-readable error message."] # [doc = ""] # [doc = " The message differs between debug and release builds. When"] # [doc = " `debug_assertions` are enabled, this message is verbose and includes"] # [doc = " potentially sensitive information."] impl < Src , Dst : ? Sized > fmt :: Display for ValidityError < Src , Dst > where Dst : KnownLayout + TryFromBytes , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("The conversion failed because the source bytes are not a valid value of the destination type.") ? ; if cfg ! (debug_assertions) { f . write_str ("\n\n") ? ; self . display_verbose_extras (f) ? ; } Ok (()) } }
};
}
