macro_rules! deps {
    () => {
        Alignment!();
        ConvertError!();
        Validity!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        # [doc = " Produces a human-readable error message."] # [doc = ""] # [doc = " The message differs between debug and release builds. When"] # [doc = " `debug_assertions` are enabled, this message is verbose and includes"] # [doc = " potentially sensitive information."] impl < A : fmt :: Display , S : fmt :: Display , V : fmt :: Display > fmt :: Display for ConvertError < A , S , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Alignment (e) => e . fmt (f) , Self :: Size (e) => e . fmt (f) , Self :: Validity (e) => e . fmt (f) , } } }
    };
}

impl_183!();