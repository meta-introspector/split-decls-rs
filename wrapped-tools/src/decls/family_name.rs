macro_rules! family_name {
    () => {
        fn family_name () -> & 'static str { if cfg ! (windows) { "windows" } else { "unix" } }
    };
}

family_name!()