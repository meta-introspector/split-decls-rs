macro_rules! other_3 {
    () => {
        # [cfg (not (feature = "windows_raw_dylib"))] pub macro link ($ ($ tt : tt) *) { $ crate :: link_dylib ! ($ ($ tt) *) ; }
    };
}

other_3!()