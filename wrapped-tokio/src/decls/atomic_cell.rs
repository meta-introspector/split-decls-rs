macro_rules! atomic_cell {
    () => {
        # [cfg (feature = "rt")] pub (crate) mod atomic_cell ;
    };
}

atomic_cell!()