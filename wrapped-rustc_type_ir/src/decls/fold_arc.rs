macro_rules! fold_arc {
    () => {
        fn fold_arc < T : Clone , E > (mut arc : Arc < T > , fold : impl FnOnce (T) -> Result < T , E > ,) -> Result < Arc < T > , E > { unsafe { Arc :: make_mut (& mut arc) ; let ptr = Arc :: into_raw (arc) . cast :: < mem :: ManuallyDrop < T > > () ; let mut unique = Arc :: from_raw (ptr) ; let slot = Arc :: get_mut (& mut unique) . unwrap_unchecked () ; let owned = mem :: ManuallyDrop :: take (slot) ; let folded = fold (owned) ? ; * slot = mem :: ManuallyDrop :: new (folded) ; Ok (Arc :: from_raw (Arc :: into_raw (unique) . cast ())) } }
    };
}

fold_arc!();