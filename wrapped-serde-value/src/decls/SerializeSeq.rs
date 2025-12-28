macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeSeq {
    () => {
        deps!();
        struct SerializeSeq (Vec < Value >) ;
    };
}

SerializeSeq!();