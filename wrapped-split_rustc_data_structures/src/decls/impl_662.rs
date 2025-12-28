macro_rules! deps {
    () => {
        UnordMap!();
        Index!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        impl < K , Q : ? Sized , V > Index < & Q > for UnordMap < K , V > where K : Eq + Hash + Borrow < Q > , Q : Eq + Hash , { type Output = V ; # [inline] fn index (& self , key : & Q) -> & V { & self . inner [key] } }
    };
}

impl_662!()