macro_rules! deps {
    () => {
        AssertDropped!();
        Slab!();
    };
}

macro_rules! remove_local {
    () => {
        deps!();
        # [test] fn remove_local () { run_model ("remove_local" , | | { let slab = Arc :: new (Slab :: new_with_config :: < TinyConfig > ()) ; let slab2 = slab . clone () ; let (dropped , item) = AssertDropped :: new (1) ; let idx = slab . insert (item) . expect ("insert") ; let guard = slab . get (idx) . unwrap () ; assert ! (slab . remove (idx)) ; let t1 = thread :: spawn (move | | { let g = slab2 . get (idx) ; drop (g) ; }) ; assert ! (slab . get (idx) . is_none ()) ; t1 . join () . expect ("thread 1 should not panic") ; drop (guard) ; assert ! (slab . get (idx) . is_none ()) ; dropped . assert_dropped () ; }) }
    };
}

remove_local!();