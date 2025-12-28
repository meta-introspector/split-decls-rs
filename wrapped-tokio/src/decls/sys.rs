macro_rules! sys {
    () => {
        pub (crate) mod sys { pub (crate) fn num_cpus () -> usize { 2 } }
    };
}

sys!()