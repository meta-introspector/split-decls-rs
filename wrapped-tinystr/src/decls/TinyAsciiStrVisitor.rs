macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! TinyAsciiStrVisitor {
    () => {
        deps!();
        struct TinyAsciiStrVisitor < const N : usize > { marker : PhantomData < TinyAsciiStr < N > > , }
    };
}

TinyAsciiStrVisitor!()