macro_rules! deps {
    () => {
        CachedThreadLocal!();
        CachedIterMut!();
        ThreadLocal!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T : Send > CachedThreadLocal < T > { # [doc = " Creates a new empty `CachedThreadLocal`."] # [inline] pub fn new () -> CachedThreadLocal < T > { CachedThreadLocal { inner : ThreadLocal :: new () , } } # [doc = " Returns the element for the current thread, if it exists."] # [inline] pub fn get (& self) -> Option < & T > { self . inner . get () } # [doc = " Returns the element for the current thread, or creates it if it doesn't"] # [doc = " exist."] # [inline] pub fn get_or < F > (& self , create : F) -> & T where F : FnOnce () -> T , { self . inner . get_or (create) } # [doc = " Returns the element for the current thread, or creates it if it doesn't"] # [doc = " exist. If `create` fails, that error is returned and no element is"] # [doc = " added."] # [inline] pub fn get_or_try < F , E > (& self , create : F) -> Result < & T , E > where F : FnOnce () -> Result < T , E > , { self . inner . get_or_try (create) } # [doc = " Returns a mutable iterator over the local values of all threads."] # [doc = ""] # [doc = " Since this call borrows the `ThreadLocal` mutably, this operation can"] # [doc = " be done safely---the mutable borrow statically guarantees no other"] # [doc = " threads are currently accessing their associated values."] # [inline] pub fn iter_mut (& mut self) -> CachedIterMut < '_ , T > { CachedIterMut { inner : self . inner . iter_mut () , } } # [doc = " Removes all thread-specific values from the `ThreadLocal`, effectively"] # [doc = " resetting it to its original state."] # [doc = ""] # [doc = " Since this call borrows the `ThreadLocal` mutably, this operation can"] # [doc = " be done safely---the mutable borrow statically guarantees no other"] # [doc = " threads are currently accessing their associated values."] # [inline] pub fn clear (& mut self) { self . inner . clear () ; } }
    };
}

impl_2!();