// Generated macro for impl_215 (impl)
macro_rules! Depcrate_errorimpl_215 {
() => {
// Module: crate::error
// Provides: {"impl_215"}
// Dependencies: {}
# [doc = " Produces a human-readable error message."] # [doc = ""] # [doc = " The message differs between debug and release builds. When"] # [doc = " `debug_assertions` are enabled, this message is verbose and includes"] # [doc = " potentially sensitive information."] impl < Src , Dst : ? Sized > fmt :: Display for AlignmentError < Src , Dst > where Src : Deref , Dst : KnownLayout , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("The conversion failed because the address of the source is not a multiple of the alignment of the destination type.") ? ; if cfg ! (debug_assertions) { self . display_verbose_extras (f) } else { Ok (()) } } }
};
}
