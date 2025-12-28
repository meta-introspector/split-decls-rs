macro_rules! Scan {
    () => {
        # [cfg (feature = "full")] enum Scan { Fail , Bailout , Consume , }
    };
}

Scan!();