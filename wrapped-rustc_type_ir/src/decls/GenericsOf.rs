macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! GenericsOf {
    () => {
        deps!();
        pub trait GenericsOf < I : Interner < GenericsOf = Self > > { fn count (& self) -> usize ; }
    };
}

GenericsOf!();