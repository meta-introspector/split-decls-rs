macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        # [cfg (feature = "printing")] impl IdentFragment for Index { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . index , formatter) } fn span (& self) -> Option < Span > { Some (self . span) } }
    };
}

impl_252!();