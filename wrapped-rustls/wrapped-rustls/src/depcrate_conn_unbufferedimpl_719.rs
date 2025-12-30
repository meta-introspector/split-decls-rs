// Generated macro for impl_719 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_719 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_719"}
// Dependencies: {}
impl < 'c , Data : SideData > From < TransmitTlsData < 'c , Data > > for ConnectionState < 'c , '_ , Data > { fn from (v : TransmitTlsData < 'c , Data >) -> Self { Self :: TransmitTlsData (v) } }
};
}
