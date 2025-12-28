macro_rules! numerous_macro_rules {
    () => {
        pub fn numerous_macro_rules () -> String { let path = project_root () . join ("bench_data/numerous_macro_rules") ; fs :: read_to_string (path) . unwrap () }
    };
}

numerous_macro_rules!();