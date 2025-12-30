// Generated macro for ethiopic_and_ethioaa_are_compatible (function)
macro_rules! Depcrate_calendar_erasethiopic_and_ethioaa_are_compatible {
() => {
// Module: crate::calendar::eras
// Provides: {"ethiopic_and_ethioaa_are_compatible"}
// Dependencies: {}
# [test] pub fn ethiopic_and_ethioaa_are_compatible () { use icu :: calendar :: cal :: { Ethiopian , EthiopianEraStyle } ; assert_eq ! (Date :: try_new_from_codes (Some ("aa") , 1 , Month :: new (1) . code () , 1 , Ethiopian :: new_with_era_style (EthiopianEraStyle :: AmeteAlem)) . unwrap () . era_year () . era_index , Date :: try_new_from_codes (Some ("aa") , 1 , Month :: new (1) . code () , 1 , Ethiopian :: new_with_era_style (EthiopianEraStyle :: AmeteMihret)) . unwrap () . era_year () . era_index ,) ; }
};
}
