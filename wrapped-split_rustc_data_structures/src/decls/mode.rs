macro_rules! mode {
    () => {
        mod mode { use std :: sync :: atomic :: { AtomicU8 , Ordering } ; const UNINITIALIZED : u8 = 0 ; const DYN_NOT_THREAD_SAFE : u8 = 1 ; const DYN_THREAD_SAFE : u8 = 2 ; static DYN_THREAD_SAFE_MODE : AtomicU8 = AtomicU8 :: new (UNINITIALIZED) ; # [inline] pub fn is_dyn_thread_safe () -> bool { match DYN_THREAD_SAFE_MODE . load (Ordering :: Relaxed) { DYN_NOT_THREAD_SAFE => false , DYN_THREAD_SAFE => true , _ => panic ! ("uninitialized dyn_thread_safe mode!") , } } # [inline] pub (super) fn might_be_dyn_thread_safe () -> bool { DYN_THREAD_SAFE_MODE . load (Ordering :: Relaxed) != DYN_NOT_THREAD_SAFE } pub fn set_dyn_thread_safe_mode (mode : bool) { let set : u8 = if mode { DYN_THREAD_SAFE } else { DYN_NOT_THREAD_SAFE } ; let previous = DYN_THREAD_SAFE_MODE . compare_exchange (UNINITIALIZED , set , Ordering :: Relaxed , Ordering :: Relaxed ,) ; assert ! (previous . is_ok () || previous == Err (set)) ; } }
    };
}

mode!()