macro_rules! deps {
    () => {
        FileEncoder!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [cfg (debug_assertions)] impl Drop for FileEncoder { fn drop (& mut self) { if ! std :: thread :: panicking () { assert ! (self . finished) ; } } }
    };
}

impl_119!()