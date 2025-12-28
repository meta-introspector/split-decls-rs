use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Parsed interface guid attribute"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[windows_interface::interface(\"8CEEB155-2849-4ce5-9448-91FF70E1E4D9\")]"] # [doc = "                              //^ parses this"] # [doc = " unsafe trait IUIAnimationVariable: IUnknown {"] # [doc = "     fn GetValue(&self, value: *mut f64) -> HRESULT;"] # [doc = " }"] # [doc = " ```"] struct Guid (Option < syn :: LitStr >) ;
}