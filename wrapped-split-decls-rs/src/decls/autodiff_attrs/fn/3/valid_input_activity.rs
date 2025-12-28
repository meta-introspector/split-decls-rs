use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn valid_input_activity (mode : DiffMode , activity : DiffActivity) -> bool { use DiffActivity :: * ; return match mode { DiffMode :: Error => false , DiffMode :: Source => false , DiffMode :: Forward => activity . is_dual_or_const () , DiffMode :: Reverse => { matches ! (activity , Active | ActiveOnly | Duplicated | DuplicatedOnly | Const) } } ; }