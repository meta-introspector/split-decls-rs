// Generated macro for impl_22 (impl)
macro_rules! Depcrate_msgs_alertimpl_22 {
() => {
// Module: crate::msgs::alert
// Provides: {"impl_22"}
// Dependencies: {}
impl Codec < '_ > for AlertMessagePayload { fn encode (& self , bytes : & mut Vec < u8 >) { self . level . encode (bytes) ; self . description . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let level = AlertLevel :: read (r) ? ; let description = AlertDescription :: read (r) ? ; r . expect_empty ("AlertMessagePayload") . map (| _ | Self { level , description }) } }
};
}
