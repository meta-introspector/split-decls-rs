macro_rules! deps {
    () => {
        Config!();
        Slab!();
        RefCount!();
    };
}

macro_rules! max_refs {
    () => {
        deps!();
        # [test] fn max_refs () { struct LargeGenConfig ; impl crate :: cfg :: Config for LargeGenConfig { const INITIAL_PAGE_SIZE : usize = 2 ; const MAX_THREADS : usize = 32 ; const MAX_PAGES : usize = 2 ; } let mut model = loom :: model :: Builder :: new () ; model . max_branches = 100000 ; model . check (| | { let slab = Slab :: new_with_config :: < LargeGenConfig > () ; let key = slab . insert ("hello world") . unwrap () ; let max = crate :: page :: slot :: RefCount :: < LargeGenConfig > :: MAX ; let mut refs = (0 .. max) . map (| _ | slab . get (key) . unwrap ()) . collect :: < Vec < _ > > () ; assert ! (slab . get (key) . is_none ()) ; drop (refs . pop ()) ; let ref1 = slab . get (key) ; assert ! (ref1 . is_some ()) ; assert ! (slab . get (key) . is_none ()) ; }) }
    };
}

max_refs!()