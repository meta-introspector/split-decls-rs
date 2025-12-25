use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This provides the inputs to the `gen_*` functions, which generate the proc macro output.
struct ImplementInputs {
    /// The user's type that was marked with `#[implement]`.
    original_type: syn::ItemStruct,
    /// The identifier for the user's original type definition.
    original_ident: syn::Ident,
    /// The list of interface chains that this type implements.
    interface_chains: Vec<InterfaceChain>,
    /// The "trust level", which is returned by `IInspectable::GetTrustLevel`.
    trust_level: usize,
    /// Determines whether `IAgileObject` and `IMarshal` are implemented automatically.
    agile: bool,
    /// The identifier of the `Foo_Impl` type.
    impl_ident: syn::Ident,
    /// The list of constraints needed for this `Foo_Impl` type.
    constraints: proc_macro2::TokenStream,
    /// The list of generic parameters for this `Foo_Impl` type, including `<` and `>`.
    /// If there are no generics, this contains `<>`.
    generics: proc_macro2::TokenStream,
    /// True if the user type has any generic parameters.
    is_generic: bool,
}
