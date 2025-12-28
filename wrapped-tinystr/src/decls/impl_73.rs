macro_rules! deps {
    () => {
        TinyAsciiStrVisitor!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < const N : usize > TinyAsciiStrVisitor < N > { fn new () -> Self { TinyAsciiStrVisitor { marker : PhantomData , } } }
    };
}

impl_73!();