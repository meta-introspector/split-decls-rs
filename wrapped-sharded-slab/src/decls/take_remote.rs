macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! take_remote {
    () => {
        deps!();
        # [test] fn take_remote () { run_model ("take_remote" , | | { let slab = Arc :: new (Slab :: new ()) ; let idx1 = slab . insert (1) . expect ("insert") ; assert_eq ! (slab . get (idx1) . unwrap () , 1) ; let idx2 = slab . insert (2) . expect ("insert") ; assert_eq ! (slab . get (idx2) . unwrap () , 2) ; let idx3 = slab . insert (3) . expect ("insert") ; assert_eq ! (slab . get (idx3) . unwrap () , 3) ; let s = slab . clone () ; let t1 = thread :: spawn (move | | { assert_eq ! (s . get (idx2) . unwrap () , 2) ; assert_eq ! (s . take (idx2) , Some (2)) ; }) ; let s = slab . clone () ; let t2 = thread :: spawn (move | | { assert_eq ! (s . get (idx3) . unwrap () , 3) ; assert_eq ! (s . take (idx3) , Some (3)) ; }) ; t1 . join () . expect ("thread 1 should not panic") ; t2 . join () . expect ("thread 2 should not panic") ; assert_eq ! (slab . get (idx1) . unwrap () , 1) ; assert ! (slab . get (idx2) . is_none ()) ; assert ! (slab . get (idx3) . is_none ()) ; }) ; }
    };
}

take_remote!()