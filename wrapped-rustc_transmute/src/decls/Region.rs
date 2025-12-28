macro_rules! Region {
    () => {
        pub (crate) trait Region : Debug + Hash + Eq + PartialEq + Copy + Clone { }
    };
}

Region!();