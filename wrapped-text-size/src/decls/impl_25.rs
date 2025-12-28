macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl TryFrom < usize > for TextSize { type Error = TryFromIntError ; # [inline] fn try_from (value : usize) -> Result < Self , TryFromIntError > { Ok (u32 :: try_from (value) ? . into ()) } }
    };
}

impl_25!();