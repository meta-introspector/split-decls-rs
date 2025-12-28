macro_rules! deps {
    () => {
        UnordItems!();
        UnordBag!();
    };
}

macro_rules! impl_668 {
    () => {
        deps!();
        impl < T , I : Iterator < Item = T > > From < UnordItems < T , I > > for UnordBag < T > { fn from (value : UnordItems < T , I >) -> Self { UnordBag { inner : Vec :: from_iter (value . 0) } } }
    };
}

impl_668!();