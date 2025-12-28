macro_rules! deps {
    () => {
        QueryJob!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < I > Clone for QueryJob < I > { fn clone (& self) -> Self { Self { id : self . id , span : self . span , parent : self . parent , latch : self . latch . clone () } } }
    };
}

impl_171!();