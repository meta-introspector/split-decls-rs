macro_rules! deps {
    () => {
        TestGraph!();
        Maxes!();
        Sccs!();
    };
}

macro_rules! test_bug_max_leak {
    () => {
        deps!();
        # [test] fn test_bug_max_leak () { let graph = TestGraph :: new (8 , & [(0 , 0) , (0 , 18) , (0 , 19) , (0 , 1) , (0 , 2) , (0 , 7) , (0 , 8) , (0 , 23) , (18 , 0) , (18 , 12) , (19 , 0) , (19 , 25) , (12 , 18) , (12 , 3) , (12 , 5) , (3 , 12) , (3 , 21) , (3 , 22) , (5 , 13) , (21 , 3) , (22 , 3) , (13 , 5) , (13 , 4) , (4 , 13) , (4 , 0) , (2 , 11) , (7 , 6) , (6 , 20) , (20 , 6) , (8 , 17) , (17 , 9) , (9 , 16) , (16 , 26) , (26 , 15) , (15 , 10) , (10 , 14) , (14 , 27) , (23 , 24) ,] ,) ; let mut annotations = Maxes :: new (| w | match w { 22 => 1 , 24 => 2 , 27 => 2 , _ => 0 , }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . annotation (sccs . scc (2)) , 0) ; assert_eq ! (annotations . annotation (sccs . scc (7)) , 0) ; assert_eq ! (annotations . annotation (sccs . scc (8)) , 2) ; assert_eq ! (annotations . annotation (sccs . scc (23)) , 2) ; assert_eq ! (annotations . annotation (sccs . scc (3)) , 2) ; assert_eq ! (annotations . annotation (sccs . scc (0)) , 2) ; }
    };
}

test_bug_max_leak!();