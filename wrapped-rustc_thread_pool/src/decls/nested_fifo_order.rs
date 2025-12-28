macro_rules! nested_fifo_order {
    () => {
        # [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_fifo_order () { let vec = test_nested_order ! (scope_fifo => spawn_fifo , scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 100) . collect () ; assert_eq ! (vec , expected) ; }
    };
}

nested_fifo_order!()