macro_rules! deps {
    () => {
        WorkProductId!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl WorkProductId { pub fn from_cgu_name (cgu_name : & str) -> WorkProductId { let mut hasher = StableHasher :: new () ; cgu_name . hash (& mut hasher) ; WorkProductId { hash : hasher . finish () } } }
    };
}

impl_27!();