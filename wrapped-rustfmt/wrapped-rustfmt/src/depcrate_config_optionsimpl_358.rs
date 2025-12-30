// Generated macro for impl_358 (impl)
macro_rules! Depcrate_config_optionsimpl_358 {
() => {
// Module: crate::config::options
// Provides: {"impl_358"}
// Dependencies: {}
impl PartialOrd for StyleEdition { fn partial_cmp (& self , other : & StyleEdition) -> Option < std :: cmp :: Ordering > { match (self , other) { (Self :: Edition2027 , Self :: Edition2027) => Some (std :: cmp :: Ordering :: Equal) , (_ , Self :: Edition2027) => Some (std :: cmp :: Ordering :: Less) , (Self :: Edition2027 , _) => Some (std :: cmp :: Ordering :: Greater) , (Self :: Edition2015 | Self :: Edition2018 | Self :: Edition2021 | Self :: Edition2024 , _) => { rustc_span :: edition :: Edition :: partial_cmp (& (* self) . into () , & (* other) . into ()) } } } }
};
}
