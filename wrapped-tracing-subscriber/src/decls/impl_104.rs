macro_rules! deps {
    () => {
        Layered!();
        Context!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < L , S > Layered < L , S > where S : Subscriber , { fn ctx (& self) -> Context < '_ , S > { Context :: new (& self . inner) } }
    };
}

impl_104!()