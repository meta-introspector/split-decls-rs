macro_rules! deps {
    () => {
        SizeError!();
        KnownLayout!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        # [doc = " Produces a human-readable error message."] # [doc = ""] # [doc = " The message differs between debug and release builds. When"] # [doc = " `debug_assertions` are enabled, this message is verbose and includes"] # [doc = " potentially sensitive information."] impl < Src , Dst : ? Sized > fmt :: Display for SizeError < Src , Dst > where Src : Deref , Dst : KnownLayout , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("The conversion failed because the source was incorrectly sized to complete the conversion into the destination type.") ? ; if cfg ! (debug_assertions) { f . write_str ("\n") ? ; self . display_verbose_extras (f) ? ; } Ok (()) } }
    };
}

impl_202!()