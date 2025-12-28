macro_rules! deps {
    () => {
        Default!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Default { pub fn is_none (& self) -> bool { match self { Default :: None => true , Default :: Default | Default :: Path (_) => false , } } }
    };
}

impl_36!();