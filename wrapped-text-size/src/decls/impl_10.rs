macro_rules! deps {
    () => {
        TextSize!();
        TextRange!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T > From < TextRange > for Range < T > where T : From < TextSize > , { # [inline] fn from (r : TextRange) -> Self { r . start () . into () .. r . end () . into () } }
    };
}

impl_10!()