// Generated macro for BindingInfo (struct)
macro_rules! DepcrateBindingInfo {
() => {
// Module: crate
// Provides: {"BindingInfo"}
// Dependencies: {}
# [doc = " Information about a specific binding. This contains both an `Ident`"] # [doc = " reference to the given field, and the syn `&'a Field` descriptor for that"] # [doc = " field."] # [doc = ""] # [doc = " This type supports `quote::ToTokens`, so can be directly used within the"] # [doc = " `quote!` macro. It expands to a reference to the matched field."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct BindingInfo < 'a > { # [doc = " The name which this `BindingInfo` will bind to."] pub binding : Ident , # [doc = " The type of binding which this `BindingInfo` will create."] pub style : BindStyle , field : & 'a Field , generics : & 'a Generics , seen_generics : Vec < bool > , index : usize , }
};
}
