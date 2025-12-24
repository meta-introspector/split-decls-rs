use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parsed interface
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
/// //^ parses this
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
struct Interface {
    visibility: syn::Visibility,
    name: syn::Ident,
    parent: Option<syn::Path>,
    methods: Vec<InterfaceMethod>,
    docs: Vec<syn::Attribute>,
}
