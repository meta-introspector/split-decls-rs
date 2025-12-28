macro_rules! deps {
    () => {
        DocumentMut!();
        Table!();
        Item!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Default for DocumentMut { fn default () -> Self { Self { root : Item :: Table (Table :: with_pos (Some (0))) , trailing : Default :: default () , } } }
    };
}

impl_39!();