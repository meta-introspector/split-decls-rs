// Generated macro for impl_592 (impl)
macro_rules! Depcrate_tableimpl_592 {
() => {
// Module: crate::table
// Provides: {"impl_592"}
// Dependencies: {}
impl ser :: SerializeMap for SerializeMap { type Ok = Table ; type Error = crate :: ser :: Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { match Value :: try_from (key) ? { Value :: String (s) => self . next_key = Some (s) , _ => return Err (crate :: ser :: Error :: key_not_string ()) , } ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { let key = self . next_key . take () ; let key = key . expect ("serialize_value called before serialize_key") ; match Value :: try_from (value) { Ok (value) => { self . map . insert (key , value) ; } Err (crate :: ser :: Error { inner : crate :: ser :: ErrorInner :: UnsupportedNone , }) => { } Err (e) => return Err (e) , } Ok (()) } fn end (self) -> Result < Table , crate :: ser :: Error > { Ok (self . map) } }
};
}
