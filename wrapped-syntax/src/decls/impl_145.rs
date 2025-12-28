macro_rules! deps {
    () => {
        SyntaxAnnotation!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl Default for SyntaxAnnotation { fn default () -> Self { static COUNTER : AtomicU32 = AtomicU32 :: new (1) ; let id = COUNTER . fetch_add (1 , Ordering :: Relaxed) ; Self (NonZeroU32 :: new (id) . expect ("syntax annotation id overflow")) } }
    };
}

impl_145!();