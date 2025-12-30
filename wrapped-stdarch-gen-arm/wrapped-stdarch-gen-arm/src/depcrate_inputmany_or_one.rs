// Generated macro for many_or_one (module)
macro_rules! Depcrate_inputmany_or_one {
() => {
// Module: crate::input
// Provides: {"many_or_one"}
// Dependencies: {}
mod many_or_one { use serde :: { Deserialize , Serialize , de :: Deserializer , ser :: Serializer } ; pub fn serialize < T , S > (vec : & Vec < T > , serializer : S) -> Result < S :: Ok , S :: Error > where T : Serialize , S : Serializer , { if vec . len () == 1 { vec . first () . unwrap () . serialize (serializer) } else { vec . serialize (serializer) } } pub fn deserialize < 'de , T , D > (deserializer : D) -> Result < Vec < T > , D :: Error > where T : Deserialize < 'de > , D : Deserializer < 'de > , { # [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged)] enum ManyOrOne < T > { Many (Vec < T >) , One (T) , } match ManyOrOne :: deserialize (deserializer) ? { ManyOrOne :: Many (vec) => Ok (vec) , ManyOrOne :: One (val) => Ok (vec ! [val]) , } } }
};
}
