// Generated macro for impl_716 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_716 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_716"}
// Dependencies: {}
impl < 'c , 'i , Data : SideData > From < ReadTraffic < 'c , 'i , Data > > for ConnectionState < 'c , 'i , Data > { fn from (v : ReadTraffic < 'c , 'i , Data >) -> Self { Self :: ReadTraffic (v) } }
};
}
