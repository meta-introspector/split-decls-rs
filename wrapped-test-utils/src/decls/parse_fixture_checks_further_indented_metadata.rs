macro_rules! deps {
    () => {
        FixtureWithProjectMeta!();
    };
}

macro_rules! parse_fixture_checks_further_indented_metadata {
    () => {
        deps!();
        # [test] # [should_panic] fn parse_fixture_checks_further_indented_metadata () { FixtureWithProjectMeta :: parse (r"
        //- /lib.rs
          mod bar;

          fn foo() {}
          //- /bar.rs
          pub fn baz() {}
          " ,) ; }
    };
}

parse_fixture_checks_further_indented_metadata!()