// Generated macro for impl_987 (impl)
macro_rules! Depcrate_configimpl_987 {
() => {
// Module: crate::config
// Provides: {"impl_987"}
// Dependencies: {}
# [cfg (any (feature = "dist-client" , feature = "dist-server"))] impl HTTPUrl { pub fn from_url (u : reqwest :: Url) -> Self { HTTPUrl (u) } pub fn to_url (& self) -> reqwest :: Url { self . 0 . clone () } }
};
}
