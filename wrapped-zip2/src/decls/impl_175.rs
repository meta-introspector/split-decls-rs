macro_rules! deps {
    () => {
        DataAndPosition!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < T > From < (T , u64) > for DataAndPosition < T > { fn from (value : (T , u64)) -> Self { Self { data : value . 0 , position : value . 1 , } } }
    };
}

impl_175!()