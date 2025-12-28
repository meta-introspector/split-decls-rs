macro_rules! deps {
    () => {
        LineInfo!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl LineInfo { pub fn from (lines : (usize , usize , usize , usize)) -> Self { LineInfo { start_line : lines . 0 , start_col : lines . 1 , end_line : lines . 2 , end_col : lines . 3 } } }
    };
}

impl_324!()