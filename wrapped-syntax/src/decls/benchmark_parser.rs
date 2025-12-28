macro_rules! benchmark_parser {
    () => {
        # [test] fn benchmark_parser () { if std :: env :: var ("RUN_SLOW_BENCHES") . is_err () { return ; } let data = bench_fixture :: glorious_old_parser () ; let tree = { let _b = bench ("parsing") ; let p = SourceFile :: parse (& data , Edition :: CURRENT) ; assert ! (p . errors () . is_empty ()) ; assert_eq ! (p . tree () . syntax . text_range () . len () , 352474 . into ()) ; p . tree () } ; { let _b = bench ("tree traversal") ; let fn_names = tree . syntax () . descendants () . filter_map (ast :: Fn :: cast) . filter_map (| f | f . name ()) . count () ; assert_eq ! (fn_names , 268) ; } }
    };
}

benchmark_parser!()