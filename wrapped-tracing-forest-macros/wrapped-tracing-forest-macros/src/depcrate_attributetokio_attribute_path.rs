// Generated macro for tokio_attribute_path (function)
macro_rules! Depcrate_attributetokio_attribute_path {
() => {
// Module: crate::attribute
// Provides: {"tokio_attribute_path"}
// Dependencies: {}
# [cfg (feature = "sync")] fn tokio_attribute_path (is_test : bool) -> syn :: Path { let mut segments = syn :: punctuated :: Punctuated :: new () ; segments . push (ident ("tokio") . into ()) ; segments . push (ident (if is_test { "test" } else { "main" }) . into ()) ; syn :: Path { leading_colon : None , segments , } }
};
}
