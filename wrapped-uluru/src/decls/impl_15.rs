macro_rules! deps {
    () => {
        LRUCache!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T , const N : usize > Default for LRUCache < T , N > { fn default () -> Self { Self :: new () } }
    };
}

impl_15!()