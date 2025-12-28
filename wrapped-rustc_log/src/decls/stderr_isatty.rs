macro_rules! stderr_isatty {
    () => {
        pub fn stderr_isatty () -> bool { io :: stderr () . is_terminal () }
    };
}

stderr_isatty!();