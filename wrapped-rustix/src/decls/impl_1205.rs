macro_rules! deps {
    () => {
        SpecialCodeIndex!();
        SpecialCodes!();
    };
}

macro_rules! impl_1205 {
    () => {
        deps!();
        impl core :: ops :: IndexMut < SpecialCodeIndex > for SpecialCodes { fn index_mut (& mut self , index : SpecialCodeIndex) -> & mut Self :: Output { & mut self . 0 [index . 0] } }
    };
}

impl_1205!();