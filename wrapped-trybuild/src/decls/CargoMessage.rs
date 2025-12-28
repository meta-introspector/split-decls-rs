macro_rules! deps {
    () => {
        Reason!();
        RustcTarget!();
        RustcMessage!();
    };
}

macro_rules! CargoMessage {
    () => {
        deps!();
        # [derive (Deserialize)] struct CargoMessage { # [allow (dead_code)] reason : Reason , target : RustcTarget , message : RustcMessage , }
    };
}

CargoMessage!()