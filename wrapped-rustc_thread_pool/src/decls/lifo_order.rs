macro_rules! lifo_order {
    () => {
        # [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn lifo_order () { let vec = test_order ! (spawn , spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
    };
}

lifo_order!();