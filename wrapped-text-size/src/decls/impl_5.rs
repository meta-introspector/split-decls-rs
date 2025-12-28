macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Index < TextRange > for str { type Output = str ; # [inline] fn index (& self , index : TextRange) -> & str { & self [Range :: < usize > :: from (index)] } }
    };
}

impl_5!();