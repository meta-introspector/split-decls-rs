// Generated macro for parse_era_start_date (function)
macro_rules! Depcrate_cldr_serde_erasparse_era_start_date {
() => {
// Module: crate::cldr_serde::eras
// Provides: {"parse_era_start_date"}
// Dependencies: {}
fn parse_era_start_date < 'de , D : Deserializer < 'de > > (de : D ,) -> Result < Option < EraStartDate > , D :: Error > { let s = Cow :: < str > :: deserialize (de) ? ; let mut s = & * s ; let sign = if let Some (suffix) = s . strip_prefix ('-') { s = suffix ; - 1 } else { 1 } ; let mut split = s . split ('-') ; let year = split . next () . ok_or (D :: Error :: custom ("EraStartData format")) ? . parse :: < i32 > () . map_err (| _ | D :: Error :: custom ("EraStartData format")) ? * sign ; let month = split . next () . ok_or (D :: Error :: custom ("EraStartData format")) ? . parse () . map_err (| _ | D :: Error :: custom ("EraStartData format")) ? ; let day = split . next () . ok_or (D :: Error :: custom ("EraStartData format")) ? . parse () . map_err (| _ | D :: Error :: custom ("EraStartData format")) ? ; Ok (Some (EraStartDate { year , month , day })) }
};
}
