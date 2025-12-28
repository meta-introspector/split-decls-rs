macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! DebugValue {
    () => {
        deps!();
        # [doc = " A `Value` which serializes as a string using `fmt::Debug`."] # [derive (Clone)] pub struct DebugValue < T : fmt :: Debug > (T) ;
    };
}

DebugValue!()