macro_rules! deps {
    () => {
        QueryCache!();
        Value!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < K , V > QueryCache for VecCache < K , V , DepNodeIndex > where K : Idx + Eq + Hash + Copy + Debug , V : Copy , { type Key = K ; type Value = V ; # [inline (always)] fn lookup (& self , key : & K) -> Option < (V , DepNodeIndex) > { self . lookup (key) } # [inline] fn complete (& self , key : K , value : V , index : DepNodeIndex) { self . complete (key , value , index) } fn iter (& self , f : & mut dyn FnMut (& Self :: Key , & Self :: Value , DepNodeIndex)) { self . iter (f) } }
    };
}

impl_199!()