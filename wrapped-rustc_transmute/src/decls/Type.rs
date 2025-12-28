macro_rules! Type {
    () => {
        pub (crate) trait Type : Debug + Hash + Eq + PartialEq + Copy + Clone { }
    };
}

Type!();