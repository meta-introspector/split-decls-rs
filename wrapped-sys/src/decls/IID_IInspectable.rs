macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! IID_IInspectable {
    () => {
        deps!();
        pub const IID_IInspectable : GUID = GUID :: from_u128 (0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90) ;
    };
}

IID_IInspectable!();