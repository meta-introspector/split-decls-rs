macro_rules! Downcast {
    () => {
        # [doc = " Methods for downcasting from an `Any`-like trait object."] # [doc = ""] # [doc = " This should only be implemented on trait objects for subtraits of `Any`, though you can"] # [doc = " implement it for other types and it'll work fine, so long as your implementation is correct."] pub trait Downcast { # [doc = " Gets the `TypeId` of `self`."] fn type_id (& self) -> TypeId ; # [doc = " Downcast from `&Any` to `&T`, without checking the type matches."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `T` matches the trait object, on pain of *undefined behavior*."] unsafe fn downcast_unchecked_ref < T : 'static > (& self) -> & T ; # [doc = " Downcast from `&mut Any` to `&mut T`, without checking the type matches."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `T` matches the trait object, on pain of *undefined behavior*."] unsafe fn downcast_unchecked_mut < T : 'static > (& mut self) -> & mut T ; }
    };
}

Downcast!();