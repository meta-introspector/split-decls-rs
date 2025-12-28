macro_rules! Def {
    () => {
        pub (crate) trait Def : Debug + Hash + Eq + PartialEq + Copy + Clone { fn has_safety_invariants (& self) -> bool ; }
    };
}

Def!();