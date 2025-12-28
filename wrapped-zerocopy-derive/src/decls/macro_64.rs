macro_rules! macro_64 {
    () => {
        derive ! (KnownLayout => derive_known_layout => derive_known_layout_inner) ;
    };
}

macro_64!();