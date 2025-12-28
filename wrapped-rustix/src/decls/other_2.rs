macro_rules! other_2 {
    () => {
        # [cfg (all (test , static_assertions))] # [macro_use] # [allow (unused_imports)] extern crate static_assertions ;
    };
}

other_2!()