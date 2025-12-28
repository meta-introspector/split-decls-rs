macro_rules! other_2 {
    () => {
        # [cfg (feature = "windows_raw_dylib")] pub macro link ($ ($ tt : tt) *) { $ crate :: link_raw_dylib ! ($ ($ tt) *) ; }
    };
}

other_2!();