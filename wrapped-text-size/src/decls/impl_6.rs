macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Index < TextRange > for String { type Output = str ; # [inline] fn index (& self , index : TextRange) -> & str { & self [Range :: < usize > :: from (index)] } }
    };
}

impl_6!()