macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! _Test {
    () => {
        deps!();
        # [cfg (test)] struct _Test where Error : Send + Sync ;
    };
}

_Test!()