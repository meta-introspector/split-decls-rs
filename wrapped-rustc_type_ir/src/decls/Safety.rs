macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! Safety {
    () => {
        deps!();
        pub trait Safety < I : Interner < Safety = Self > > : Copy + Debug + Hash + Eq { fn safe () -> Self ; fn is_safe (self) -> bool ; fn prefix_str (self) -> & 'static str ; }
    };
}

Safety!();