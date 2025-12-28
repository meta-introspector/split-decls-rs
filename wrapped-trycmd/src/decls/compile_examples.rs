macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! compile_examples {
    () => {
        deps!();
        # [doc = " Prepare all examples for testing"] # [doc = ""] # [doc = " Unlike `cargo_bin!`, this does not inherit all of the current compiler settings.  It"] # [doc = " will match the current target and profile but will not get feature flags.  Pass those arguments"] # [doc = " to the compiler via `args`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " #[test]"] # [doc = " fn cli_tests() {"] # [doc = "     trycmd::TestCases::new()"] # [doc = "         .register_bins(trycmd::cargo::compile_examples([]).unwrap())"] # [doc = "         .case(\"examples/cmd/*.trycmd\");"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "examples")] pub fn compile_examples < 'a > (args : impl IntoIterator < Item = & 'a str > ,) -> Result < impl Iterator < Item = (String , crate :: schema :: Bin) > , crate :: Error > { snapbox :: cmd :: compile_examples (args) . map (| i | i . map (| (name , path) | (name , path . into ()))) }
    };
}

compile_examples!()