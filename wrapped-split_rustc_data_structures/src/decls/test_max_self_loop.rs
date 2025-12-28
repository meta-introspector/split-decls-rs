macro_rules! deps {
    () => {
        TestGraph!();
        Maxes!();
        Sccs!();
    };
}

macro_rules! test_max_self_loop {
    () => {
        deps!();
        # [test] fn test_max_self_loop () { let graph = TestGraph :: new (0 , & [(0 , 0)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | if n == 0 { 17 } else { 0 }) ; Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . 0 [0] , 17) ; }
    };
}

test_max_self_loop!();