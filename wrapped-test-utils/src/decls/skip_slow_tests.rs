macro_rules! skip_slow_tests {
    () => {
        # [doc = " Returns `false` if slow tests should not run, otherwise returns `true` and"] # [doc = " also creates a file at `./target/.slow_tests_cookie` which serves as a flag"] # [doc = " that slow tests did run."] pub fn skip_slow_tests () -> bool { let should_skip = (std :: env :: var ("CI") . is_err () && std :: env :: var ("RUN_SLOW_TESTS") . is_err ()) || std :: env :: var ("SKIP_SLOW_TESTS") . is_ok () ; if should_skip { eprintln ! ("ignoring slow test") ; } else { let path = target_dir () . join (".slow_tests_cookie") ; fs :: write (path , ".") . unwrap () ; } should_skip }
    };
}

skip_slow_tests!()