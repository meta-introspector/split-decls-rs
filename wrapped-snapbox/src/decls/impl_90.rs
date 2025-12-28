macro_rules! deps {
    () => {
        DataSourceInner!();
        DataSource!();
        Inline!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl From < Inline > for DataSource { fn from (inline : Inline) -> Self { Self { inner : DataSourceInner :: Inline (inline) , } } }
    };
}

impl_90!();