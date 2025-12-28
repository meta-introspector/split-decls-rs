macro_rules! deps {
    () => {
        HStringHeader!();
        RefCount!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl HStringHeader { pub fn alloc (len : u32) -> * mut Self { if len == 0 { return core :: ptr :: null_mut () ; } let bytes = core :: mem :: size_of :: < Self > () + 2 * len as usize ; let header = unsafe { bindings :: HeapAlloc (bindings :: GetProcessHeap () , 0 , bytes) } as * mut Self ; if header . is_null () { panic ! ("allocation failed") ; } unsafe { header . write (core :: mem :: MaybeUninit :: < Self > :: zeroed () . assume_init ()) ; (* header) . len = len ; (* header) . count = RefCount :: new (1) ; (* header) . data = & mut (* header) . buffer_start ; } header } pub unsafe fn free (header : * mut Self) { if header . is_null () { return ; } unsafe { bindings :: HeapFree (bindings :: GetProcessHeap () , 0 , header as * mut _) ; } } pub fn duplicate (& self) -> * mut Self { if self . flags & HSTRING_REFERENCE_FLAG == 0 { self . count . add_ref () ; self as * const Self as * mut Self } else { let copy = Self :: alloc (self . len) ; unsafe { core :: ptr :: copy_nonoverlapping (self . data , (* copy) . data , self . len as usize + 1) ; } copy } } }
    };
}

impl_79!()