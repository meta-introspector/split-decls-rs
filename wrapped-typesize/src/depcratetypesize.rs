// Generated macro for TypeSize (trait)
macro_rules! DepcrateTypeSize {
() => {
// Module: crate
// Provides: {"TypeSize"}
// Dependencies: {}
# [doc = " A trait to fetch an accurate estimate of the total memory usage of a value."] # [doc = ""] # [doc = " Unless you are writing a data structure, you should derive this trait using [`derive::TypeSize`]."] # [doc = ""] # [doc = " Note: Implementations cannot be relied on for any form of `unsafe` bound,"] # [doc = " as this is entirely safe to implement incorrectly."] pub trait TypeSize : Sized { # [doc = " The number of bytes more than the [`core::mem::size_of`] that this value is using."] # [must_use] fn extra_size (& self) -> usize { 0 } # [doc = " The total number of bytes that this type is using, both direct"] # [doc = " ([`core::mem::size_of`]) and indirect (behind allocations)"] # [doc = ""] # [doc = " There's no reason to ever override this method."] # [must_use] fn get_size (& self) -> usize { core :: mem :: size_of :: < Self > () + self . extra_size () } # [doc = " Returns information about the number of items this type is holding, if it is a collection."] # [must_use] # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { None } # [doc = " Returns detailed information about the current value's field sizes."] # [doc = ""] # [doc = " This should generally be implemented by [`derive::TypeSize`]"] # [must_use] # [cfg (feature = "details")] fn get_size_details (& self) -> alloc :: vec :: Vec < Field > { alloc :: vec :: Vec :: new () } }
};
}
