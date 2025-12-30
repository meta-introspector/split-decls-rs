// Generated macro for impl_21 (impl)
macro_rules! Depcrate_block_apiimpl_21 {
() => {
// Module: crate::block_api
// Provides: {"impl_21"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > SerializableState for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { type SerializedStateSize = U200 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState :: < Self > :: default () ; let chunks = serialized_state . chunks_exact_mut (8) ; for (val , chunk) in self . state . iter () . zip (chunks) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_state } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let mut state = [0 ; PLEN] ; let chunks = serialized_state . chunks_exact (8) ; for (val , chunk) in state . iter_mut () . zip (chunks) { * val = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } Ok (Self { state , _pd : PhantomData , }) } }
};
}
