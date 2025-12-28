macro_rules! deps {
    () => {
        TinierConfig!();
        Slab!();
    };
}

macro_rules! concurrent_remove_remote_and_reuse {
    () => {
        deps!();
        # [test] fn concurrent_remove_remote_and_reuse () { let mut model = loom :: model :: Builder :: new () ; model . max_branches = 100000 ; run_builder ("concurrent_remove_remote_and_reuse" , model , | | { let slab = Arc :: new (Slab :: new_with_config :: < TinierConfig > ()) ; let idx1 = slab . insert (1) . unwrap () ; let idx2 = slab . insert (2) . unwrap () ; assert_eq ! (slab . get (idx1) . unwrap () , 1 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx2) . unwrap () , 2 , "slab: {:#?}" , slab) ; let s = slab . clone () ; let s2 = slab . clone () ; let t1 = thread :: spawn (move | | { s . take (idx1) . expect ("must remove") ; }) ; let t2 = thread :: spawn (move | | { s2 . take (idx2) . expect ("must remove") ; }) ; let idx3 = store_when_free (& slab , 3) ; t1 . join () . expect ("thread 1 should not panic") ; t2 . join () . expect ("thread 1 should not panic") ; assert ! (slab . get (idx1) . is_none () , "slab: {:#?}" , slab) ; assert ! (slab . get (idx2) . is_none () , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx3) . unwrap () , 3 , "slab: {:#?}" , slab) ; }) ; }
    };
}

concurrent_remove_remote_and_reuse!()