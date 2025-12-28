macro_rules! deps {
    () => {
        RowId!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Default for RowId { fn default () -> Self { Self { val : - 1 } } }
    };
}

impl_19!();