macro_rules! should_skip_all_archive_creation {
    () => {
        fn should_skip_all_archive_creation () -> bool { cfg ! (windows) || (is_ci :: cached () && env :: var_os ("GIX_TEST_CREATE_ARCHIVES_EVEN_ON_CI") . is_none ()) }
    };
}

should_skip_all_archive_creation!()