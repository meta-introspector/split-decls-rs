// Generated macro for Parameters (struct)
macro_rules! Depcrate_deParameters {
() => {
// Module: crate::de
// Provides: {"Parameters"}
// Dependencies: {}
struct Parameters { # [doc = " Name of the type the `derive` is on."] local : syn :: Ident , # [doc = " Path to the type the impl is for. Either a single `Ident` for local"] # [doc = " types (does not include generic parameters) or `some::remote::Path` for"] # [doc = " remote types."] this_type : syn :: Path , # [doc = " Same as `this_type` but using `::<T>` for generic parameters for use in"] # [doc = " expression position."] this_value : syn :: Path , # [doc = " Generics including any explicit and inferred bounds for the impl."] generics : syn :: Generics , # [doc = " Lifetimes borrowed from the deserializer. These will become bounds on"] # [doc = " the `'de` lifetime of the deserializer."] borrowed : BorrowedLifetimes , # [doc = " At least one field has a serde(getter) attribute, implying that the"] # [doc = " remote type has a private field."] has_getter : bool , # [doc = " Type has a repr(packed) attribute."] is_packed : bool , }
};
}
