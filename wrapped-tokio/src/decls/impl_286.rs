macro_rules! deps {
    () => {
        MetricAtomicU64!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        # [allow (dead_code)] impl MetricAtomicU64 { cfg_64bit_metrics ! { pub (crate) fn load (& self , ordering : Ordering) -> u64 { self . value . load (ordering) } } cfg_64bit_metrics ! { pub (crate) fn store (& self , val : u64 , ordering : Ordering) { self . value . store (val , ordering) } pub (crate) fn new (value : u64) -> Self { Self { value : AtomicU64 :: new (value) } } pub (crate) fn add (& self , value : u64 , ordering : Ordering) { self . value . fetch_add (value , ordering) ; } } cfg_no_64bit_metrics ! { pub (crate) fn store (& self , _val : u64 , _ordering : Ordering) { } pub (crate) fn add (& self , _value : u64 , _ordering : Ordering) { } pub (crate) fn new (_value : u64) -> Self { Self { } } } }
    };
}

impl_286!()