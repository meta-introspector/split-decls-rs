macro_rules! deps {
    () => {
        DropTest!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a > Drop for DropTest < 'a > { fn drop (& mut self) { self . counter . fetch_sub (1 , Ordering :: Relaxed) ; } }
    };
}

impl_119!()