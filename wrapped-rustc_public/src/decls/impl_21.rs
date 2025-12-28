macro_rules! deps {
    () => {
        Layout!();
        LayoutShape!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Layout { pub fn shape (self) -> LayoutShape { with (| cx | cx . layout_shape (self)) } }
    };
}

impl_21!()