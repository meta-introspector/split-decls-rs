macro_rules! deps {
    () => {
        CacheAligned!();
        Sharded!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < T > Sharded < T > { # [inline] pub fn new (mut value : impl FnMut () -> T) -> Self { if is_dyn_thread_safe () { return Sharded :: Shards (Box :: new ([() ; SHARDS] . map (| () | CacheAligned (Lock :: new (value ()))) ,)) ; } Sharded :: Single (Lock :: new (value ())) } # [doc = " The shard is selected by hashing `val` with `FxHasher`."] # [inline] pub fn get_shard_by_value < K : Hash + ? Sized > (& self , val : & K) -> & Lock < T > { match self { Self :: Single (single) => single , Self :: Shards (..) => self . get_shard_by_hash (make_hash (val)) , } } # [inline] pub fn get_shard_by_hash (& self , hash : u64) -> & Lock < T > { self . get_shard_by_index (get_shard_hash (hash)) } # [inline] pub fn get_shard_by_index (& self , i : usize) -> & Lock < T > { match self { Self :: Single (single) => single , Self :: Shards (shards) => { unsafe { & shards . get_unchecked (i & (SHARDS - 1)) . 0 } } } } # [doc = " The shard is selected by hashing `val` with `FxHasher`."] # [inline] # [track_caller] pub fn lock_shard_by_value < K : Hash + ? Sized > (& self , val : & K) -> LockGuard < '_ , T > { match self { Self :: Single (single) => { unsafe { single . lock_assume (Mode :: NoSync) } } Self :: Shards (..) => self . lock_shard_by_hash (make_hash (val)) , } } # [inline] # [track_caller] pub fn lock_shard_by_hash (& self , hash : u64) -> LockGuard < '_ , T > { self . lock_shard_by_index (get_shard_hash (hash)) } # [inline] # [track_caller] pub fn lock_shard_by_index (& self , i : usize) -> LockGuard < '_ , T > { match self { Self :: Single (single) => { unsafe { single . lock_assume (Mode :: NoSync) } } Self :: Shards (shards) => { unsafe { shards . get_unchecked (i & (SHARDS - 1)) . 0 . lock_assume (Mode :: Sync) } } } } # [inline] pub fn lock_shards (& self) -> impl Iterator < Item = LockGuard < '_ , T > > { match self { Self :: Single (single) => Either :: Left (iter :: once (single . lock ())) , Self :: Shards (shards) => Either :: Right (shards . iter () . map (| shard | shard . 0 . lock ())) , } } # [inline] pub fn try_lock_shards (& self) -> impl Iterator < Item = Option < LockGuard < '_ , T > > > { match self { Self :: Single (single) => Either :: Left (iter :: once (single . try_lock ())) , Self :: Shards (shards) => Either :: Right (shards . iter () . map (| shard | shard . 0 . try_lock ())) , } } }
    };
}

impl_390!()