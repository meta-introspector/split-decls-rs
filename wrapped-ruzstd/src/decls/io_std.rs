macro_rules! io_std {
    () => {
        # [cfg (feature = "std")] pub mod io_std ;
    };
}

io_std!();