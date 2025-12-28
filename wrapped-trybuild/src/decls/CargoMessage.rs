macro_rules! deps {
    () => {
        Reason!();
        RustcMessage!();
        RustcTarget!();
    };
}

macro_rules! CargoMessage {
    () => {
        deps!();
        # [derive (Deserialize)] struct CargoMessage { # [allow (dead_code)] reason : Reason , target : RustcTarget , message : RustcMessage , }
    };
}

CargoMessage!();