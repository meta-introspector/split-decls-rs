macro_rules! impl_78 {
    () => {
        impl < S > StreamNotifyClose < S > { # [doc = " Create a new `StreamNotifyClose`."] pub fn new (stream : S) -> Self { Self { inner : Some (stream) , } } # [doc = " Get back the inner `Stream`."] # [doc = ""] # [doc = " Returns `None` if the stream has reached its end."] pub fn into_inner (self) -> Option < S > { self . inner } }
    };
}

impl_78!();