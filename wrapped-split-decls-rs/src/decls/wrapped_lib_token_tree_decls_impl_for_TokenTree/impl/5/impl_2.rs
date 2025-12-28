use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TokenTree { # [doc = " Returns `true` if the given token tree is delimited."] pub fn is_delimited (& self) -> bool { matches ! (* self , TokenTree :: Delimited (..)) } # [doc = " Returns `true` if the given token tree is a token of the given kind."] pub fn is_token (& self , expected_kind : & TokenKind) -> bool { match self { TokenTree :: Token (Token { kind : actual_kind , .. }) => actual_kind == expected_kind , _ => false , } } # [doc = " Retrieves the `TokenTree`'s span."] pub fn span (& self) -> Span { match * self { TokenTree :: Token (Token { span , .. }) | TokenTree :: MetaVar (span , _) | TokenTree :: MetaVarDecl { span , .. } => span , TokenTree :: Delimited (span , ..) | TokenTree :: MetaVarExpr (span , _) | TokenTree :: Sequence (span , _) => span . entire () , } } pub fn token (kind : TokenKind , span : Span) -> TokenTree { TokenTree :: Token (Token :: new (kind , span)) } }
}