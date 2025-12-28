macro_rules! chrono {
    () => {
        # [cfg (feature = "chrono")] mod chrono ;
    };
}

chrono!()