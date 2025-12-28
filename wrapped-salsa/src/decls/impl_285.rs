macro_rules! deps {
    () => {
        StorageHandle!();
        CoordinateDrop!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl < Db > Clone for StorageHandle < Db > { fn clone (& self) -> Self { Self { zalsa_impl : self . zalsa_impl . clone () , coordinate : CoordinateDrop (Arc :: clone (& self . coordinate)) , phantom : PhantomData , } } }
    };
}

impl_285!();