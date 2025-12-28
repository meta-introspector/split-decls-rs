macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! compile_example {
    () => {
        deps!();
        # [doc = " Prepare an example for testing"] # [doc = ""] # [doc = " Unlike `cargo_bin!`, this does not inherit all of the current compiler settings.  It"] # [doc = " will match the current target and profile but will not get feature flags.  Pass those arguments"] # [doc = " to the compiler via `args`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " #[test]"] # [doc = " fn cli_tests() {"] # [doc = "     trycmd::TestCases::new()"] # [doc = "         .register_bin(\"example-fixture\", trycmd::cargo::compile_example(\"example-fixture\", []))"] # [doc = "         .case(\"examples/cmd/*.trycmd\");"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "examples")] pub fn compile_example < 'a > (target_name : & str , args : impl IntoIterator < Item = & 'a str > ,) -> crate :: schema :: Bin { snapbox :: cmd :: compile_example (target_name , args) . into () }
    };
}

compile_example!();