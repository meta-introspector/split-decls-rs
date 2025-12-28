macro_rules! parking_lot {
    () => {
        # [cfg (all (feature = "parking_lot" , not (miri)))] mod parking_lot ;
    };
}

parking_lot!()