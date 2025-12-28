macro_rules! deps {
    () => {
        InflateStream!();
        ReturnCode!();
        Flags!();
    };
}

macro_rules! undermine {
    () => {
        deps!();
        pub fn undermine (stream : & mut InflateStream , subvert : i32) -> ReturnCode { stream . state . flags . update (Flags :: SANE , (! subvert) != 0) ; ReturnCode :: Ok }
    };
}

undermine!();