use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Remove any \"mut\" from a method argument's binding."] fn demutify_arg (arg : & mut PatType) { match * arg . pat { Pat :: Wild (_) => { compile_error (arg . span () , "Mocked methods must have named arguments") ; } Pat :: Ident (ref mut pat_ident) => { if let Some (r) = & pat_ident . by_ref { compile_error (r . span () , "Mockall does not support by-reference argument bindings" ,) ; } if let Some ((_at , subpat)) = & pat_ident . subpat { compile_error (subpat . span () , "Mockall does not support subpattern bindings" ,) ; } pat_ident . mutability = None ; } _ => { compile_error (arg . span () , "Unsupported argument type") ; } } ; }
}