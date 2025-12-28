macro_rules! RefutableFlag {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq)] enum RefutableFlag { Irrefutable , Refutable , }
    };
}

RefutableFlag!()