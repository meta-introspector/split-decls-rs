macro_rules! deps {
    () => {
        QueryCache!();
        Value!();
        DefaultCache!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < K , V > QueryCache for DefaultCache < K , V > where K : Eq + Hash + Copy + Debug , V : Copy , { type Key = K ; type Value = V ; # [inline (always)] fn lookup (& self , key : & K) -> Option < (V , DepNodeIndex) > { self . cache . get (key) } # [inline] fn complete (& self , key : K , value : V , index : DepNodeIndex) { self . cache . insert (key , (value , index)) ; } fn iter (& self , f : & mut dyn FnMut (& Self :: Key , & Self :: Value , DepNodeIndex)) { for shard in self . cache . lock_shards () { for (k , v) in shard . iter () { f (k , & v . 0 , v . 1) ; } } } }
    };
}

impl_192!();