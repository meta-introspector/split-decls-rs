macro_rules! deps {
    () => {
        QueryLatch!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < I > Clone for QueryLatch < I > { fn clone (& self) -> Self { Self { info : Arc :: clone (& self . info) } } }
    };
}

impl_177!();