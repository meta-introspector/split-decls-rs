macro_rules! stdout_isatty {
    () => {
        pub fn stdout_isatty () -> bool { io :: stdout () . is_terminal () }
    };
}

stdout_isatty!();