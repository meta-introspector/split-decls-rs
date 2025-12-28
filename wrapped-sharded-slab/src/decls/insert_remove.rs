macro_rules! deps {
    () => {
        CustomConfig!();
        Slab!();
    };
}

macro_rules! insert_remove {
    () => {
        deps!();
        # [doc = " Calls `insert(); remove()` multiple times to detect invalid releasing."] # [doc = " Initially, it revealed bugs in the `Slot::release_with()` implementation."] # [test] fn insert_remove () { eprintln ! ("bits={}; config={:#?}" , usize :: BITS , CustomConfig :: debug ()) ; let default_slab = Slab :: < u64 , _ > :: new () ; let custom_slab = Slab :: < u64 , _ > :: new_with_config :: < CustomConfig > () ; for i in 0 ..= ITERS { let idx = default_slab . insert (i) . unwrap () ; assert ! (default_slab . remove (idx)) ; let idx = custom_slab . insert (i) . unwrap () ; assert ! (custom_slab . remove (idx)) ; } slab_eq (custom_slab , default_slab) ; }
    };
}

insert_remove!()