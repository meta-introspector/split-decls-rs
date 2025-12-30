// Generated macro for persistence (module)
macro_rules! Depcrate_zalsa_localpersistence {
() => {
// Module: crate::zalsa_local
// Provides: {"persistence"}
// Dependencies: {}
# [cfg (feature = "persistence")] pub (crate) mod persistence { use super :: { QueryOrigin , QueryRevisions , QueryRevisionsExtra } ; use crate :: sync :: atomic :: { AtomicBool , Ordering } ; use crate :: { Durability , Revision } ; # [doc = " A reference to the fields of [`QueryRevisions`], with its [`QueryOrigin`] transformed."] # [derive (serde :: Serialize)] pub (crate) struct MappedQueryRevisions < 'a > { changed_at : Revision , durability : Durability , origin : QueryOrigin , # [serde (with = "verified_final")] verified_final : AtomicBool , extra : & 'a QueryRevisionsExtra , } impl QueryRevisions { pub (crate) fn with_origin (& self , origin : QueryOrigin) -> MappedQueryRevisions < '_ > { let QueryRevisions { changed_at , durability , ref verified_final , ref extra , # [cfg (feature = "accumulator")] accumulated_inputs : _ , origin : _ , } = * self ; MappedQueryRevisions { changed_at , durability , extra , origin , verified_final : AtomicBool :: new (verified_final . load (Ordering :: Relaxed)) , } } } pub (super) mod verified_final { use crate :: sync :: atomic :: { AtomicBool , Ordering } ; pub fn serialize < S > (value : & AtomicBool , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& value . load (Ordering :: Relaxed) , serializer) } pub fn deserialize < 'de , D > (deserializer : D) -> Result < AtomicBool , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (AtomicBool :: new) } } }
};
}
