macro_rules! write_failure_marker {
    () => {
        fn write_failure_marker (failure_marker : & Path) { std :: fs :: write (failure_marker , []) . ok () ; }
    };
}

write_failure_marker!()