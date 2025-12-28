macro_rules! deps {
    () => {
        Table!();
        Item!();
        Document!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Default for Document < & 'static str > { fn default () -> Self { Self { root : Item :: Table (Table :: with_pos (Some (0))) , trailing : Default :: default () , raw : "" , } } }
    };
}

impl_34!()