use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Parse for WriteInput { fn parse (input : ParseStream) -> syn :: parse :: Result < Self > { let dst : Expr = input . parse () ? ; let _ : Comma = input . parse () ? ; let rest = input . parse_terminated (Expr :: parse , Comma) ? ; let rest = quote ! { # rest } . into () ; Ok (Self { dst , rest }) } }