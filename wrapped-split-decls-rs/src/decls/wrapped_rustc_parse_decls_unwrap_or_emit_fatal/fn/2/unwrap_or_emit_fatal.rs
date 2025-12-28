use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn unwrap_or_emit_fatal < T > (expr : Result < T , Vec < Diag < '_ > > >) -> T { match expr { Ok (expr) => expr , Err (errs) => { for err in errs { err . emit () ; } FatalError . raise () } } }