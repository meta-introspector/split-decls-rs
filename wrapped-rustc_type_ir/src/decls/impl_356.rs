macro_rules! deps {
    () => {
        ImplPolarity!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl ImplPolarity { # [doc = " The polarity marker in front of the impl trait ref if applicable."] pub fn as_str (self) -> & 'static str { match self { Self :: Positive => "" , Self :: Negative => "!" , Self :: Reservation => "" , } } }
    };
}

impl_356!();