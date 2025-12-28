macro_rules! deps {
    () => {
        RefMut!();
        Ref!();
        Config!();
        Clear!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a , T , C : cfg :: Config > RefMut < 'a , T , C > where T : Clear + Default , C : cfg :: Config , { # [doc = " Returns the key used to access the guard."] pub fn key (& self) -> usize { self . key } # [doc = " Downgrades the mutable guard to an immutable guard, allowing access to"] # [doc = " the pooled value from other threads."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use sharded_slab::Pool;"] # [doc = " # use std::{sync::Arc, thread};"] # [doc = " let pool = Arc::new(Pool::<String>::new());"] # [doc = ""] # [doc = " let mut guard_mut = pool.clone().create_owned().unwrap();"] # [doc = " let key = guard_mut.key();"] # [doc = " guard_mut.push_str(\"Hello\");"] # [doc = ""] # [doc = " // The pooled string is currently borrowed mutably, so other threads"] # [doc = " // may not access it."] # [doc = " let pool2 = pool.clone();"] # [doc = " thread::spawn(move || {"] # [doc = "     assert!(pool2.get(key).is_none())"] # [doc = " }).join().unwrap();"] # [doc = ""] # [doc = " // Downgrade the guard to an immutable reference."] # [doc = " let guard = guard_mut.downgrade();"] # [doc = ""] # [doc = " // Now, other threads may also access the pooled value."] # [doc = " let pool2 = pool.clone();"] # [doc = " thread::spawn(move || {"] # [doc = "     let guard = pool2.get(key)"] # [doc = "         .expect(\"the item may now be referenced by other threads\");"] # [doc = "     assert_eq!(guard, String::from(\"Hello\"));"] # [doc = " }).join().unwrap();"] # [doc = ""] # [doc = " // We can still access the value immutably through the downgraded guard."] # [doc = " assert_eq!(guard, String::from(\"Hello\"));"] # [doc = " ```"] pub fn downgrade (mut self) -> Ref < 'a , T , C > { let inner = unsafe { self . inner . downgrade () } ; Ref { inner , shard : self . shard , key : self . key , } } # [inline] fn value (& self) -> & T { unsafe { self . inner . value () } } }
    };
}

impl_22!();