use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn convert_implements_to_interface_chains(implements: Vec<ImplementType>) -> Vec<InterfaceChain> {
    let mut chains = Vec::with_capacity(implements.len());
    for (i, implement) in implements.into_iter().enumerate() {
        let mut ident_string = format!("interface{}", i + 1);
        let suffix = get_interface_ident_suffix(&implement.type_name);
        if !suffix.is_empty() {
            ident_string.push('_');
            ident_string.push_str(&suffix);
        }
        let field_ident = syn::Ident::new(&ident_string, implement.span);
        let mut vtable_const_string = ident_string.clone();
        vtable_const_string.make_ascii_uppercase();
        vtable_const_string.insert_str(0, "VTABLE_");
        let vtable_const_ident = syn::Ident::new(&vtable_const_string, implement.span);
        chains.push(InterfaceChain {
            implement,
            field_ident,
            vtable_const_ident,
        });
    }
    chains
}
