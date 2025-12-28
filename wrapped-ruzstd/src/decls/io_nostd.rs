macro_rules! io_nostd {
    () => {
        # [cfg (not (feature = "std"))] pub mod io_nostd ;
    };
}

io_nostd!();