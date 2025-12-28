macro_rules! Weak {
    () => {
        pub (crate) struct Weak < F > { name : & 'static str , addr : AtomicPtr < c_void > , _marker : marker :: PhantomData < F > , }
    };
}

Weak!();