macro_rules! glorious_old_parser {
    () => {
        pub fn glorious_old_parser () -> String { let path = project_root () . join ("bench_data/glorious_old_parser") ; fs :: read_to_string (path) . unwrap () }
    };
}

glorious_old_parser!();