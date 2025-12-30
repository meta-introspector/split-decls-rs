// Generated macro for impl_718 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_718 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_718"}
// Dependencies: {}
impl < 'c , Data : SideData > From < EncodeTlsData < 'c , Data > > for ConnectionState < 'c , '_ , Data > { fn from (v : EncodeTlsData < 'c , Data >) -> Self { Self :: EncodeTlsData (v) } }
};
}
