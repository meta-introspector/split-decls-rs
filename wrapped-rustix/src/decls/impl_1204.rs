macro_rules! deps {
    () => {
        SpecialCodes!();
        SpecialCodeIndex!();
    };
}

macro_rules! impl_1204 {
    () => {
        deps!();
        impl core :: ops :: Index < SpecialCodeIndex > for SpecialCodes { type Output = u8 ; fn index (& self , index : SpecialCodeIndex) -> & Self :: Output { & self . 0 [index . 0] } }
    };
}

impl_1204!()