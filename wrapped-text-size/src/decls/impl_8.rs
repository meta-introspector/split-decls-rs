macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl IndexMut < TextRange > for String { # [inline] fn index_mut (& mut self , index : TextRange) -> & mut str { & mut self [Range :: < usize > :: from (index)] } }
    };
}

impl_8!();