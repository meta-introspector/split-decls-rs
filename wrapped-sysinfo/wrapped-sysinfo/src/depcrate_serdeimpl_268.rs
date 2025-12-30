// Generated macro for impl_268 (impl)
macro_rules! Depcrate_serdeimpl_268 {
() => {
// Module: crate::serde
// Provides: {"impl_268"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: ProcessStatus { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let (index , variant , maybe_value) = match * self { Self :: Idle => (0 , "Idle" , None) , Self :: Run => (1 , "Run" , None) , Self :: Sleep => (2 , "Sleep" , None) , Self :: Stop => (3 , "Stop" , None) , Self :: Zombie => (4 , "Zombie" , None) , Self :: Tracing => (5 , "Tracing" , None) , Self :: Dead => (6 , "Dead" , None) , Self :: Wakekill => (7 , "Wakekill" , None) , Self :: Waking => (8 , "Waking" , None) , Self :: Parked => (9 , "Parked" , None) , Self :: LockBlocked => (10 , "LockBlocked" , None) , Self :: UninterruptibleDiskSleep => (11 , "UninterruptibleDiskSleep" , None) , Self :: Unknown (n) => (12 , "Unknown" , Some (n)) , } ; if let Some (ref value) = maybe_value { serializer . serialize_newtype_variant ("ProcessStatus" , index , variant , value) } else { serializer . serialize_unit_variant ("ProcessStatus" , index , variant) } } }
};
}
