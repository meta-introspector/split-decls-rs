macro_rules! deps {
    () => {
        WeakSliceMut!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < 'a , T > WeakSliceMut < 'a , T > { # [doc = " # Safety"] # [doc = ""] # [doc = " The arguments must satisfy the requirements of [`core::slice::from_raw_parts_mut`]. The"] # [doc = " difference versus a slice is that the slice requirements are only enforced when a slice is"] # [doc = " needed, so in practice we mostly get the bounds checking and other convenient slice APIs,"] # [doc = " without the exact correctness constraints of a rust core/std slice."] pub (crate) unsafe fn from_raw_parts_mut (ptr : * mut T , len : usize) -> Self { Self { ptr , len , _marker : PhantomData , } } pub (crate) fn into_raw_parts (self) -> (* mut T , usize) { (self . ptr , self . len) } pub (crate) fn as_slice (& self) -> & 'a [T] { unsafe { core :: slice :: from_raw_parts (self . ptr , self . len) } } pub (crate) fn as_mut_slice (& mut self) -> & 'a mut [T] { unsafe { core :: slice :: from_raw_parts_mut (self . ptr , self . len) } } pub (crate) fn as_ptr (& self) -> * const T { self . ptr } pub (crate) fn as_mut_ptr (& mut self) -> * mut T { self . ptr } pub (crate) fn len (& self) -> usize { self . len } pub (crate) fn is_empty (& self) -> bool { self . len () == 0 } pub (crate) fn empty () -> Self { let buf = & mut [] ; Self { ptr : buf . as_mut_ptr () , len : buf . len () , _marker : PhantomData , } } }
    };
}

impl_267!()