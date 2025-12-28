use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Parsed interface"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[windows_interface::interface(\"8CEEB155-2849-4ce5-9448-91FF70E1E4D9\")]"] # [doc = " unsafe trait IUIAnimationVariable: IUnknown {"] # [doc = " //^ parses this"] # [doc = "     fn GetValue(&self, value: *mut f64) -> HRESULT;"] # [doc = " }"] # [doc = " ```"] struct Interface { visibility : syn :: Visibility , name : syn :: Ident , parent : Option < syn :: Path > , methods : Vec < InterfaceMethod > , docs : Vec < syn :: Attribute > , }