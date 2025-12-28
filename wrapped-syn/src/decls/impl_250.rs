macro_rules! impl_250 {
    () => {
        impl PartialEq for Index { fn eq (& self , other : & Self) -> bool { self . index == other . index } }
    };
}

impl_250!()