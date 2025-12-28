macro_rules! deps {
    () => {
        Config!();
        OwnedRefMut!();
        OwnedRef!();
        Shard!();
        Clear!();
        Tid!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T , C > OwnedRefMut < T , C > where T : Clear + Default , C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [doc = " Downgrades the owned mutable guard to an owned immutable guard, allowing"] # [doc = " access to the pooled value from other threads."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use sharded_slab::Pool;"] # [doc = " # use std::{sync::Arc, thread};"] # [doc = " let pool = Arc::new(Pool::<String>::new());"] # [doc = ""] # [doc = " let mut guard_mut = pool.clone().create_owned().unwrap();"] # [doc = " let key = guard_mut.key();"] # [doc = " guard_mut.push_str(\"Hello\");"] # [doc = ""] # [doc = " // The pooled string is currently borrowed mutably, so other threads"] # [doc = " // may not access it."] # [doc = " let pool2 = pool.clone();"] # [doc = " thread::spawn(move || {"] # [doc = "     assert!(pool2.get(key).is_none())"] # [doc = " }).join().unwrap();"] # [doc = ""] # [doc = " // Downgrade the guard to an immutable reference."] # [doc = " let guard = guard_mut.downgrade();"] # [doc = ""] # [doc = " // Now, other threads may also access the pooled value."] # [doc = " let pool2 = pool.clone();"] # [doc = " thread::spawn(move || {"] # [doc = "     let guard = pool2.get(key)"] # [doc = "         .expect(\"the item may now be referenced by other threads\");"] # [doc = "     assert_eq!(guard, String::from(\"Hello\"));"] # [doc = " }).join().unwrap();"] # [doc = ""] # [doc = " // We can still access the value immutably through the downgraded guard."] # [doc = " assert_eq!(guard, String::from(\"Hello\"));"] # [doc = " ```"] pub fn downgrade (mut self) -> OwnedRef < T , C > { let inner = unsafe { self . inner . downgrade () } ; OwnedRef { inner , pool : self . pool . clone () , key : self . key , } } fn shard (& self) -> Option < & Shard < T , C > > { let shard_idx = Tid :: < C > :: from_packed (self . key) ; test_println ! ("-> shard={:?}" , shard_idx) ; self . pool . shards . get (shard_idx . as_usize ()) } # [inline] fn value (& self) -> & T { unsafe { self . inner . value () } } }
    };
}

impl_35!();