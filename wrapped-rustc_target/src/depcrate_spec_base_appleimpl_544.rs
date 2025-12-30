// Generated macro for impl_544 (impl)
macro_rules! Depcrate_spec_base_appleimpl_544 {
() => {
// Module: crate::spec::base::apple
// Provides: {"impl_544"}
// Dependencies: {}
impl FromStr for OSVersion { type Err = ParseIntError ; # [doc = " Parse an OS version triple (SDK version or deployment target)."] fn from_str (version : & str) -> Result < Self , ParseIntError > { if let Some ((major , minor)) = version . split_once ('.') { let major = major . parse () ? ; if let Some ((minor , patch)) = minor . split_once ('.') { Ok (Self { major , minor : minor . parse () ? , patch : patch . parse () ? }) } else { Ok (Self { major , minor : minor . parse () ? , patch : 0 }) } } else { Ok (Self { major : version . parse () ? , minor : 0 , patch : 0 }) } } }
};
}
