macro_rules! deps {
    () => {
        CustomConfig!();
        Slab!();
    };
}

macro_rules! double_get {
    () => {
        deps!();
        # [doc = " Calls `get()` multiple times to detect invalid ref counting."] # [doc = " Initially, it revealed bugs in the `Slot::get()` implementation."] # [test] fn double_get () { eprintln ! ("bits={}; config={:#?}" , usize :: BITS , CustomConfig :: debug ()) ; let default_slab = Slab :: < u64 , _ > :: new () ; let custom_slab = Slab :: < u64 , _ > :: new_with_config :: < CustomConfig > () ; for i in 0 ..= ITERS { let idx = default_slab . insert (i) . unwrap () ; assert ! (default_slab . get (idx) . is_some ()) ; assert ! (default_slab . get (idx) . is_some ()) ; assert ! (default_slab . remove (idx)) ; let idx = custom_slab . insert (i) . unwrap () ; assert ! (custom_slab . get (idx) . is_some ()) ; assert ! (custom_slab . get (idx) . is_some ()) ; assert ! (custom_slab . remove (idx)) ; } slab_eq (custom_slab , default_slab) ; }
    };
}

double_get!();