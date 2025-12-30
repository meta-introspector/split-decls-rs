// Generated macro for sealed (module)
macro_rules! Depcrate_writesealed {
() => {
// Module: crate::write
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use std :: sync :: Arc ; use super :: ExtendedFileOptions ; pub trait Sealed { } # [doc = " File options Extensions"] # [doc (hidden)] pub trait FileOptionExtension : Default + Sealed { # [doc = " Extra Data"] fn extra_data (& self) -> Option < & Arc < Vec < u8 > > > ; # [doc = " Central Extra Data"] fn central_extra_data (& self) -> Option < & Arc < Vec < u8 > > > ; } impl Sealed for () { } impl FileOptionExtension for () { fn extra_data (& self) -> Option < & Arc < Vec < u8 > > > { None } fn central_extra_data (& self) -> Option < & Arc < Vec < u8 > > > { None } } impl Sealed for ExtendedFileOptions { } impl FileOptionExtension for ExtendedFileOptions { fn extra_data (& self) -> Option < & Arc < Vec < u8 > > > { Some (& self . extra_data) } fn central_extra_data (& self) -> Option < & Arc < Vec < u8 > > > { Some (& self . central_extra_data) } } }
};
}
