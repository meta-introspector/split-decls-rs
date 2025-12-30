// Generated macro for impl_152 (impl)
macro_rules! Depcrate_common_networkimpl_152 {
() => {
// Module: crate::common::network
// Provides: {"impl_152"}
// Dependencies: {}
impl FromStr for MacAddr { type Err = MacAddrFromStrError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut parts = s . split (':') . map (| s | u8 :: from_str_radix (s , 16) . map_err (MacAddrFromStrError :: IntError)) ; let Some (data0) = parts . next () else { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } ; let Some (data1) = parts . next () else { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } ; let Some (data2) = parts . next () else { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } ; let Some (data3) = parts . next () else { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } ; let Some (data4) = parts . next () else { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } ; let Some (data5) = parts . next () else { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } ; if parts . next () . is_some () { return Err (MacAddrFromStrError :: InvalidAddrFormat) ; } Ok (MacAddr ([data0 ? , data1 ? , data2 ? , data3 ? , data4 ? , data5 ?])) } }
};
}
