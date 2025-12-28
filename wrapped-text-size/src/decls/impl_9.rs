macro_rules! deps {
    () => {
        TextRange!();
        TextSize!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl RangeBounds < TextSize > for TextRange { fn start_bound (& self) -> Bound < & TextSize > { Bound :: Included (& self . start) } fn end_bound (& self) -> Bound < & TextSize > { Bound :: Excluded (& self . end) } }
    };
}

impl_9!()