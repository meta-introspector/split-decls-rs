// Generated macro for impl_71 (impl)
macro_rules! Depcrate_tagimpl_71 {
() => {
// Module: crate::tag
// Provides: {"impl_71"}
// Dependencies: {}
impl < S , I > Builder < S , I > { # [doc = " Set the prefix."] pub fn prefix (self , prefix : & 'static str) -> Builder < S , I > { Builder { prefix : Some (prefix) , .. self } } # [doc = " Set the suffix."] pub fn suffix (self , suffix : & 'static str) -> Builder < Suffix , I > { Builder { prefix : self . prefix , suffix : Suffix (suffix) , icon : self . icon , } } # [doc = " Set the icon."] pub fn icon (self , icon : char) -> Builder < S , Icon > { Builder { prefix : self . prefix , suffix : self . suffix , icon : Icon (icon) , } } # [doc = " Set the suffix and icon using defaults for each [`Level`]."] # [doc = ""] # [doc = " If the `Tag` won't have a prefix, then `Tag::from(level)` can be used as"] # [doc = " a shorter alternative."] pub fn level (self , level : Level) -> Builder < Suffix , Icon > { let (suffix , icon) = match level { Level :: TRACE => ("trace" , '📍') , Level :: DEBUG => ("debug" , '🐛') , Level :: INFO => ("info" , 'ｉ') , Level :: WARN => ("warn" , '🚧') , Level :: ERROR => ("error" , '🚨') , } ; Builder { prefix : self . prefix , suffix : Suffix (suffix) , icon : Icon (icon) , } } }
};
}
