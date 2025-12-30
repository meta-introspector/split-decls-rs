// Generated macro for impl_53 (impl)
macro_rules! Depcrate_ser_pairimpl_53 {
() => {
// Module: crate::ser::pair
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'input , 'target , Target > ser :: SerializeTuple for PairSerializer < 'input , 'target , Target > where Target : 'target + UrlEncodedTarget , { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + ser :: Serialize > (& mut self , value : & T ,) -> Result < () , Error > { match mem :: replace (& mut self . state , PairState :: Done) { PairState :: WaitingForKey => { let key_sink = KeySink :: new (| key | Ok (key . into ())) ; let key_serializer = PartSerializer :: new (key_sink) ; self . state = PairState :: WaitingForValue { key : value . serialize (key_serializer) ? , } ; Ok (()) } PairState :: WaitingForValue { key } => { let result = { let value_sink = ValueSink :: new (self . urlencoder , & key) ; let value_serializer = PartSerializer :: new (value_sink) ; value . serialize (value_serializer) } ; if result . is_ok () { self . state = PairState :: Done ; } else { self . state = PairState :: WaitingForValue { key } ; } result } PairState :: Done => Err (Error :: done ()) , } } fn end (self) -> Result < () , Error > { if let PairState :: Done = self . state { Ok (()) } else { Err (Error :: not_done ()) } } }
};
}
