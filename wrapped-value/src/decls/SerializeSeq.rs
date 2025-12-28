macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! SerializeSeq {
    () => {
        deps!();
        struct SerializeSeq (Vec < ConstValue >) ;
    };
}

SerializeSeq!()