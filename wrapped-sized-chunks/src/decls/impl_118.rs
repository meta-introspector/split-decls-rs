macro_rules! deps {
    () => {
        DropTest!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a > DropTest < 'a > { pub (crate) fn new (counter : & 'a AtomicUsize) -> Self { counter . fetch_add (1 , Ordering :: Relaxed) ; DropTest { counter } } }
    };
}

impl_118!();