// Generated macro for impl_717 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_717 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_717"}
// Dependencies: {}
impl < 'c , 'i , Data : SideData > From < ReadEarlyData < 'c , 'i , Data > > for ConnectionState < 'c , 'i , Data > { fn from (v : ReadEarlyData < 'c , 'i , Data >) -> Self { Self :: ReadEarlyData (v) } }
};
}
