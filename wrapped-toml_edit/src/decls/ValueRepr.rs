macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! ValueRepr {
    () => {
        deps!();
        pub trait ValueRepr : crate :: private :: Sealed { # [doc = " The TOML representation of the value"] # [cfg (feature = "display")] fn to_repr (& self) -> Repr ; }
    };
}

ValueRepr!()