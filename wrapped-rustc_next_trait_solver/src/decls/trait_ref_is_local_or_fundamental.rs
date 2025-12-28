macro_rules! trait_ref_is_local_or_fundamental {
    () => {
        pub fn trait_ref_is_local_or_fundamental < I : Interner > (tcx : I , trait_ref : ty :: TraitRef < I >) -> bool { trait_ref . def_id . is_local () || tcx . trait_is_fundamental (trait_ref . def_id) }
    };
}

trait_ref_is_local_or_fundamental!()