macro_rules! deps {
    () => {
        FixtureWithProjectMeta!();
    };
}

macro_rules! parse_fixture_gets_full_meta {
    () => {
        deps!();
        # [test] fn parse_fixture_gets_full_meta () { let FixtureWithProjectMeta { fixture : parsed , mini_core , proc_macro_names , toolchain , target_data_layout : _ , target_arch : _ , } = FixtureWithProjectMeta :: parse (r#"
//- toolchain: nightly
//- proc_macros: identity
//- minicore: coerce_unsized
//- /lib.rs crate:foo deps:bar,baz cfg:foo=a,bar=b,atom env:OUTDIR=path/to,OTHER=foo
mod m;
"# ,) ; assert_eq ! (toolchain , Some ("nightly" . to_owned ())) ; assert_eq ! (proc_macro_names , vec ! ["identity" . to_owned ()]) ; assert_eq ! (mini_core . unwrap () . activated_flags , vec ! ["coerce_unsized" . to_owned ()]) ; assert_eq ! (1 , parsed . len ()) ; let meta = & parsed [0] ; assert_eq ! ("mod m;\n" , meta . text) ; assert_eq ! ("foo" , meta . krate . as_ref () . unwrap ()) ; assert_eq ! ("/lib.rs" , meta . path) ; assert_eq ! (2 , meta . env . len ()) ; }
    };
}

parse_fixture_gets_full_meta!();