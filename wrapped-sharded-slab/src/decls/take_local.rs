macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! take_local {
    () => {
        deps!();
        # [test] fn take_local () { run_model ("take_local" , | | { let slab = Arc :: new (Slab :: new ()) ; let s = slab . clone () ; let t1 = thread :: spawn (move | | { let idx = s . insert (1) . expect ("insert") ; assert_eq ! (s . get (idx) . unwrap () , 1) ; assert_eq ! (s . take (idx) , Some (1)) ; assert ! (s . get (idx) . is_none ()) ; let idx = s . insert (2) . expect ("insert") ; assert_eq ! (s . get (idx) . unwrap () , 2) ; assert_eq ! (s . take (idx) , Some (2)) ; assert ! (s . get (idx) . is_none ()) ; }) ; let s = slab . clone () ; let t2 = thread :: spawn (move | | { let idx = s . insert (3) . expect ("insert") ; assert_eq ! (s . get (idx) . unwrap () , 3) ; assert_eq ! (s . take (idx) , Some (3)) ; assert ! (s . get (idx) . is_none ()) ; let idx = s . insert (4) . expect ("insert") ; assert_eq ! (s . get (idx) . unwrap () , 4) ; assert_eq ! (s . take (idx) , Some (4)) ; assert ! (s . get (idx) . is_none ()) ; }) ; let s = slab ; let idx1 = s . insert (5) . expect ("insert") ; assert_eq ! (s . get (idx1) . unwrap () , 5) ; let idx2 = s . insert (6) . expect ("insert") ; assert_eq ! (s . get (idx2) . unwrap () , 6) ; assert_eq ! (s . take (idx1) , Some (5)) ; assert ! (s . get (idx1) . is_none ()) ; assert_eq ! (s . get (idx2) . unwrap () , 6) ; assert_eq ! (s . take (idx2) , Some (6)) ; assert ! (s . get (idx2) . is_none ()) ; t1 . join () . expect ("thread 1 should not panic") ; t2 . join () . expect ("thread 2 should not panic") ; }) ; }
    };
}

take_local!();