macro_rules! deps {
    () => {
        Data!();
        IntoData!();
        Inline!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl IntoData for Inline { fn into_data (self) -> Data { let trimmed = self . trimmed () ; Data :: text (trimmed) . with_source (self) } }
    };
}

impl_129!()