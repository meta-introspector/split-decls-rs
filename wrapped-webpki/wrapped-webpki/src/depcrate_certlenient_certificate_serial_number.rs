// Generated macro for lenient_certificate_serial_number (function)
macro_rules! Depcrate_certlenient_certificate_serial_number {
() => {
// Module: crate::cert
// Provides: {"lenient_certificate_serial_number"}
// Dependencies: {}
pub (crate) fn lenient_certificate_serial_number < 'a > (input : & mut untrusted :: Reader < 'a > ,) -> Result < untrusted :: Input < 'a > , Error > { der :: expect_tag (input , Tag :: Integer) }
};
}
