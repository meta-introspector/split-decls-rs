use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Describes one COM interface chain.
struct InterfaceChain {
    /// The name of the field for the vtable chain, e.g. `interface4_ifoo`.
    field_ident: syn::Ident,
    /// The name of the associated constant item for the vtable chain's initializer,
    /// e.g. `INTERFACE4_IFOO_VTABLE`.
    vtable_const_ident: syn::Ident,
    implement: ImplementType,
}
