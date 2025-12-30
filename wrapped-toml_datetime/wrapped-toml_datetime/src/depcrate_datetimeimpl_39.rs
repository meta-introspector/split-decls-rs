// Generated macro for impl_39 (impl)
macro_rules! Depcrate_datetimeimpl_39 {
() => {
// Module: crate::datetime
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: de :: Deserialize < 'de > for Time { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { match Datetime :: deserialize (deserializer) ? { Datetime { date : None , time : Some (time) , offset : None , } => Ok (time) , datetime => Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: Other (datetime . type_name ()) , & Self :: type_name () ,)) , } } }
};
}
