// Generated macro for impl_27 (impl)
macro_rules! Depcrate_schemaimpl_27 {
() => {
// Module: crate::schema
// Provides: {"impl_27"}
// Dependencies: {}
impl std :: str :: FromStr for JoinedArgs { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let inner = shlex :: Shlex :: new (s) . collect () ; Ok (Self { inner }) } }
};
}
