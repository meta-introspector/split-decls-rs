macro_rules! deps {
    () => {
        Array!();
        Config!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > Drop for Array < T , C > { fn drop (& mut self) { let max = self . max . load (Acquire) ; for shard in & self . shards [0 ..= max] { let ptr = shard . 0 . load (Acquire) ; if ptr . is_null () { continue ; } let shard = unsafe { Box :: from_raw (ptr) } ; drop (shard) } } }
    };
}

impl_156!();