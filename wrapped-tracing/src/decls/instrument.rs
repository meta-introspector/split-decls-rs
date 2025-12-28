macro_rules! instrument {
    () => {
        # [doc = " Attach a span to a `std::future::Future`."] pub mod instrument ;
    };
}

instrument!();