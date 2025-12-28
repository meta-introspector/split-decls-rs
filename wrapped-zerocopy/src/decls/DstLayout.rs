macro_rules! deps {
    () => {
        SizeInfo!();
    };
}

macro_rules! DstLayout {
    () => {
        deps!();
        # [doc = " The layout of a type which might be dynamically-sized."] # [doc = ""] # [doc = " `DstLayout` describes the layout of sized types, slice types, and \"slice"] # [doc = " DSTs\" - ie, those that are known by the type system to have a trailing slice"] # [doc = " (as distinguished from `dyn Trait` types - such types *might* have a"] # [doc = " trailing slice type, but the type system isn't aware of it)."] # [doc = ""] # [doc = " Note that `DstLayout` does not have any internal invariants, so no guarantee"] # [doc = " is made that a `DstLayout` conforms to any of Rust's requirements regarding"] # [doc = " the layout of real Rust types or instances of types."] # [doc (hidden)] # [allow (missing_debug_implementations , missing_copy_implementations)] # [cfg_attr (any (kani , test) , derive (Debug , PartialEq , Eq))] # [derive (Copy , Clone)] pub struct DstLayout { pub (crate) align : NonZeroUsize , pub (crate) size_info : SizeInfo , pub (crate) statically_shallow_unpadded : bool , }
    };
}

DstLayout!()