macro_rules! capacity_overflow {
    () => {
        # [cold] fn capacity_overflow () -> ! { panic ! ("capacity overflow") }
    };
}

capacity_overflow!();