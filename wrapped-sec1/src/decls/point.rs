macro_rules! point {
    () => {
        # [cfg (feature = "point")] pub mod point ;
    };
}

point!()