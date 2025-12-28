macro_rules! deps {
    () => {
        FromIdWithDb!();
        Id!();
        Zalsa!();
        FromId!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T : FromId > FromIdWithDb for T { # [inline] fn from_id (id : Id , _zalsa : & Zalsa) -> Self { FromId :: from_id (id) } }
    };
}

impl_143!()