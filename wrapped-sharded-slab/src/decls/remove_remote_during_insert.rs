macro_rules! deps {
    () => {
        AssertDropped!();
        Slab!();
    };
}

macro_rules! remove_remote_during_insert {
    () => {
        deps!();
        # [test] fn remove_remote_during_insert () { run_model ("remove_remote_during_insert" , | | { let slab = Arc :: new (Slab :: new_with_config :: < TinyConfig > ()) ; let slab2 = slab . clone () ; let (dropped , item) = AssertDropped :: new (1) ; let idx = slab . insert (item) . expect ("insert") ; let t1 = thread :: spawn (move | | { let g = slab2 . get (idx) ; assert_ne ! (g . as_ref () . map (| v | v . val) , Some (2)) ; drop (g) ; }) ; let (_ , item) = AssertDropped :: new (2) ; assert ! (slab . remove (idx)) ; let idx2 = slab . insert (item) . expect ("insert") ; t1 . join () . expect ("thread 1 should not panic") ; assert ! (slab . get (idx) . is_none ()) ; assert ! (slab . get (idx2) . is_some ()) ; dropped . assert_dropped () ; }) ; }
    };
}

remove_remote_during_insert!();