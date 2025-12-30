// Generated macro for transfer (function)
macro_rules! Depcratetransfer {
() => {
// Module: crate
// Provides: {"transfer"}
// Dependencies: {}
fn transfer < L , R , LS , RS > (buffers : & mut TempBuffers , left : & mut L , right : & mut R , expect_data : Option < usize > ,) -> f64 where L : DerefMut + Deref < Target = ConnectionCommon < LS > > , R : DerefMut + Deref < Target = ConnectionCommon < RS > > , LS : SideData , RS : SideData , { let mut read_time = 0f64 ; let mut data_left = expect_data ; loop { let mut sz = 0 ; while left . wants_write () { let written = left . write_tls (& mut buffers . tls [sz ..] . as_mut ()) . unwrap () ; if written == 0 { break ; } sz += written ; } if sz == 0 { return read_time ; } let mut offs = 0 ; loop { let start = Instant :: now () ; match right . read_tls (& mut buffers . tls [offs .. sz] . as_ref ()) { Ok (read) => { right . process_new_packets () . unwrap () ; offs += read ; } Err (err) => { panic ! ("error on transfer {offs}..{sz}: {err}") ; } } if let Some (left) = & mut data_left { loop { let sz = match right . reader () . read (& mut [0u8 ; 16_384]) { Ok (sz) => sz , Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => break , Err (err) => panic ! ("failed to read data: {err}") , } ; * left -= sz ; if * left == 0 { break ; } } } let end = Instant :: now () ; read_time += duration_nanos (end . duration_since (start)) ; if sz == offs { break ; } } } }
};
}
