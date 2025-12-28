macro_rules! deps {
    () => {
        ThreadLocal!();
        RawIter!();
        Entry!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl RawIter { # [inline] fn new () -> Self { Self { yielded : 0 , bucket : 0 , bucket_size : 1 , index : 0 , } } fn next < 'a , T : Send + Sync > (& mut self , thread_local : & 'a ThreadLocal < T >) -> Option < & 'a T > { while let Some (bucket) = thread_local . buckets . get (self . bucket) { let bucket = bucket . load (Ordering :: Acquire) ; if ! bucket . is_null () { while self . index < self . bucket_size { let entry = unsafe { & * bucket . add (self . index) } ; self . index += 1 ; if let Some (value) = unsafe { entry . as_ref () } { self . yielded += 1 ; return Some (value) ; } } } self . next_bucket () ; } None } fn next_mut < 'a , T : Send > (& mut self , thread_local : & 'a mut ThreadLocal < T > ,) -> Option < & 'a mut Entry < T > > { if * thread_local . values . get_mut () == self . yielded { return None ; } loop { let bucket = unsafe { thread_local . buckets . get_unchecked_mut (self . bucket) } ; let bucket = * bucket . get_mut () ; if ! bucket . is_null () { while self . index < self . bucket_size { let entry = unsafe { & mut * bucket . add (self . index) } ; self . index += 1 ; if * entry . present . get_mut () { self . yielded += 1 ; return Some (entry) ; } } } self . next_bucket () ; } } # [inline] fn next_bucket (& mut self) { self . bucket_size <<= 1 ; self . bucket += 1 ; self . index = 0 ; } fn size_hint < T : Send > (& self , thread_local : & ThreadLocal < T >) -> (usize , Option < usize >) { let total = thread_local . values . load (Ordering :: Relaxed) ; (total . saturating_sub (self . yielded) , None) } fn size_hint_frozen < T : Send > (& self , thread_local : & ThreadLocal < T >) -> (usize , Option < usize >) { let total = thread_local . values . load (Ordering :: Relaxed) ; let remaining = total - self . yielded ; (remaining , Some (remaining)) } }
    };
}

impl_39!()