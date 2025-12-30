// Generated macro for impl_266 (impl)
macro_rules! Depcrate_serdeimpl_266 {
() => {
// Module: crate::serde
// Provides: {"impl_266"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: Signal { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let (index , variant) = match * self { Self :: Hangup => (0 , "Hangup") , Self :: Interrupt => (1 , "Interrupt") , Self :: Quit => (2 , "Quit") , Self :: Illegal => (3 , "Illegal") , Self :: Trap => (4 , "Trap") , Self :: Abort => (5 , "Abort") , Self :: IOT => (6 , "IOT") , Self :: Bus => (7 , "Bus") , Self :: FloatingPointException => (8 , "FloatingPointException") , Self :: Kill => (9 , "Kill") , Self :: User1 => (10 , "User1") , Self :: Segv => (11 , "Segv") , Self :: User2 => (12 , "User2") , Self :: Pipe => (13 , "Pipe") , Self :: Alarm => (14 , "Alarm") , Self :: Term => (15 , "Term") , Self :: Child => (16 , "Child") , Self :: Continue => (17 , "Continue") , Self :: Stop => (18 , "Stop") , Self :: TSTP => (19 , "TSTP") , Self :: TTIN => (20 , "TTIN") , Self :: TTOU => (21 , "TTOU") , Self :: Urgent => (22 , "Urgent") , Self :: XCPU => (23 , "XCPU") , Self :: XFSZ => (24 , "XFSZ") , Self :: VirtualAlarm => (25 , "VirtualAlarm") , Self :: Profiling => (26 , "Profiling") , Self :: Winch => (27 , "Winch") , Self :: IO => (28 , "IO") , Self :: Poll => (29 , "Poll") , Self :: Power => (30 , "Power") , Self :: Sys => (31 , "Sys") , } ; serializer . serialize_unit_variant ("Signal" , index , variant) } }
};
}
