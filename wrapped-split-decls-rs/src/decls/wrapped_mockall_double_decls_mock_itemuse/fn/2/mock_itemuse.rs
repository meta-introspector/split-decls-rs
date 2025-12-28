use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn mock_itemuse (orig : & mut ItemUse) { if let UseTree :: Name (un) = & orig . tree { compile_error (un . span () , "Cannot double types in the current module.  Use a submodule (use foo::Foo) or a rename (use Foo as Bar)" ,) ; } else { mock_usetree (& mut orig . tree) } }