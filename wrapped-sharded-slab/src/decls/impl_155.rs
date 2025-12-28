macro_rules! deps {
    () => {
        Config!();
        Ptr!();
        IterMut!();
        Shard!();
        Tid!();
        Array!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T , C > Array < T , C > where C : cfg :: Config , { pub (crate) fn new () -> Self { let mut shards = Vec :: with_capacity (C :: MAX_SHARDS) ; for _ in 0 .. C :: MAX_SHARDS { shards . push (Ptr :: null ()) ; } Self { shards : shards . into () , max : AtomicUsize :: new (0) , } } # [inline] pub (crate) fn get (& self , idx : usize) -> Option < & Shard < T , C > > { test_println ! ("-> get shard={}" , idx) ; self . shards . get (idx) ? . load (Acquire) } # [inline] pub (crate) fn current (& self) -> (Tid < C > , & Shard < T , C >) { let tid = Tid :: < C > :: current () ; test_println ! ("current: {:?}" , tid) ; let idx = tid . as_usize () ; assert ! (idx < self . shards . len () , "Thread count overflowed the configured max count. \
            Thread index = {}, max threads = {}." , idx , C :: MAX_SHARDS ,) ; let shard = self . shards [idx] . load (Relaxed) . unwrap_or_else (| | { let ptr = Box :: into_raw (Box :: new (alloc :: Track :: new (Shard :: new (idx)))) ; test_println ! ("-> allocated new shard for index {} at {:p}" , idx , ptr) ; self . shards [idx] . set (ptr) ; let mut max = self . max . load (Acquire) ; while max < idx { match self . max . compare_exchange (max , idx , AcqRel , Acquire) { Ok (_) => break , Err (actual) => max = actual , } } test_println ! ("-> highest index={}, prev={}" , std :: cmp :: max (max , idx) , max) ; unsafe { & * ptr } . get_ref () }) ; (tid , shard) } pub (crate) fn iter_mut (& mut self) -> IterMut < '_ , T , C > { test_println ! ("Array::iter_mut") ; let max = self . max . load (Acquire) ; test_println ! ("-> highest index={}" , max) ; IterMut (self . shards [0 ..= max] . iter_mut ()) } }
    };
}

impl_155!();