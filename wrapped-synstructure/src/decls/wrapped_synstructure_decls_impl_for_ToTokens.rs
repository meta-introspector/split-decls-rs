use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ToTokens for BindingInfo<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.binding.to_tokens(tokens);
    }
}
