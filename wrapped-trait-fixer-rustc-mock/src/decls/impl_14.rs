macro_rules! deps {
    () => {
        Item!();
        MockHir!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl MockHir { pub fn walk_tops (self , _f : impl FnMut (& Item < 'static >)) { } }
    };
}

impl_14!();