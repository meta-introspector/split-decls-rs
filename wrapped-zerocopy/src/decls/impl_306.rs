macro_rules! impl_306 {
    () => {
        impl < 'a , T : ? Sized > PtrInner < 'a , T > { # [doc = " Constructs a `PtrInner` from a reference."] # [inline] pub (crate) fn from_ref (ptr : & 'a T) -> Self { let ptr = NonNull :: from (ptr) ; unsafe { Self :: new (ptr) } } # [doc = " Constructs a `PtrInner` from a mutable reference."] # [inline] pub (crate) fn from_mut (ptr : & 'a mut T) -> Self { let ptr = NonNull :: from (ptr) ; unsafe { Self :: new (ptr) } } # [must_use] # [inline (always)] pub fn cast_sized < U > (self) -> PtrInner < 'a , U > where T : Sized , { static_assert ! (T , U => mem :: size_of ::< T > () >= mem :: size_of ::< U > ()) ; unsafe { self . cast () } } # [doc = " # Safety"] # [doc = ""] # [doc = " `U` must not be larger than the size of `self`'s referent."] # [must_use] # [inline (always)] pub unsafe fn cast < U > (self) -> PtrInner < 'a , U > { let ptr = self . as_non_null () . cast :: < U > () ; unsafe { PtrInner :: new (ptr) } } }
    };
}

impl_306!()