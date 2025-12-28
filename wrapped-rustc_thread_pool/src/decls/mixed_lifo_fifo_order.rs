macro_rules! mixed_lifo_fifo_order {
    () => {
        # [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_lifo_fifo_order () { let vec = test_mixed_order ! (spawn , spawn_fifo) ; let expected = vec ! [3 , - 1 , 2 , - 2 , 1 , - 3 , 0] ; assert_eq ! (vec , expected) ; }
    };
}

mixed_lifo_fifo_order!();