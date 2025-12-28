macro_rules! deps {
    () => {
        AtomicRef!();
    };
}

macro_rules! impl_691 {
    () => {
        deps!();
        impl < T : 'static > AtomicRef < T > { pub const fn new (initial : & 'static T) -> AtomicRef < T > { AtomicRef (AtomicPtr :: new (initial as * const T as * mut T) , PhantomData) } pub fn swap (& self , new : & 'static T) -> & 'static T { unsafe { & * self . 0 . swap (new as * const T as * mut T , Ordering :: SeqCst) } } }
    };
}

impl_691!()