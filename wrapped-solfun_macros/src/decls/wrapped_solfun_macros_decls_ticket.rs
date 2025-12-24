use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "ticket", vis = "pub", hash = "7a2e21f6")]
pub fn ticket(input: TokenStream) -> TokenStream {
    macros::ticket::ticket_impl(input)
}
