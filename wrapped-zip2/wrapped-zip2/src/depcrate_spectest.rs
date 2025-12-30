// Generated macro for test (module)
macro_rules! Depcrate_spectest {
() => {
// Module: crate::spec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use std :: io :: Cursor ; # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [repr (packed , C)] pub struct TestBlock { magic : Magic , pub file_name_length : u16 , } unsafe impl Pod for TestBlock { } impl FixedSizeBlock for TestBlock { const MAGIC : Magic = Magic :: literal (0x01111) ; fn magic (self) -> Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("unreachable") ; to_and_from_le ! [(magic , Magic) , (file_name_length , u16)] ; } # [doc = " Demonstrate that a block object can be safely written to memory and deserialized back out."] # [test] fn block_serde () { let block = TestBlock { magic : TestBlock :: MAGIC , file_name_length : 3 , } ; let mut c = Cursor :: new (Vec :: new ()) ; block . write (& mut c) . unwrap () ; c . set_position (0) ; let block2 = TestBlock :: parse (& mut c) . unwrap () ; assert_eq ! (block , block2) ; } }
};
}
