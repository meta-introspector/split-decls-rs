macro_rules! deps {
    () => {
        SendAncillaryBuffer!();
    };
}

macro_rules! impl_572 {
    () => {
        deps!();
        impl Default for SendAncillaryBuffer < '_ , '_ , '_ > { fn default () -> Self { Self { buffer : & mut [] , length : 0 , _phantom : PhantomData , } } }
    };
}

impl_572!();