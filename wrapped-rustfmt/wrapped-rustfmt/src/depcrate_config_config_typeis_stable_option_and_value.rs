// Generated macro for is_stable_option_and_value (function)
macro_rules! Depcrate_config_config_typeis_stable_option_and_value {
() => {
// Module: crate::config::config_type
// Provides: {"is_stable_option_and_value"}
// Dependencies: {}
pub (crate) fn is_stable_option_and_value < T > (option_name : & str , option_stable : bool , option_value : & T ,) -> bool where T : PartialEq + std :: fmt :: Debug + ConfigType , { let nightly = crate :: is_nightly_channel ! () ; let variant_stable = option_value . stable_variant () ; match (nightly , option_stable , variant_stable) { (false , false , _) => { eprintln ! ("Warning: can't set `{option_name} = {option_value:?}`, unstable features are only \
                       available in nightly channel.") ; false } (false , true , false) => { eprintln ! ("Warning: can't set `{option_name} = {option_value:?}`, unstable variants are only \
                       available in nightly channel.") ; false } (true , _ , _) | (false , true , true) => true , } }
};
}
