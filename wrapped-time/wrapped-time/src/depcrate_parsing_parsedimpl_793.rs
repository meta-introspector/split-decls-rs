// Generated macro for impl_793 (impl)
macro_rules! Depcrate_parsing_parsedimpl_793 {
() => {
// Module: crate::parsing::parsed
// Provides: {"impl_793"}
// Dependencies: {}
impl TryFrom < Parsed > for Time { type Error = error :: TryFromParsed ; # [inline] fn try_from (parsed : Parsed) -> Result < Self , Self :: Error > { let hour = match (parsed . hour_24 () , parsed . hour_12 () , parsed . hour_12_is_pm ()) { (Some (hour) , _ , _) => hour , (_ , Some (hour) , Some (false)) if hour . get () == 12 => 0 , (_ , Some (hour) , Some (true)) if hour . get () == 12 => 12 , (_ , Some (hour) , Some (false)) => hour . get () , (_ , Some (hour) , Some (true)) => hour . get () + 12 , _ => return Err (InsufficientInformation) , } ; if parsed . hour_24 () . is_none () && parsed . hour_12 () . is_some () && parsed . hour_12_is_pm () . is_some () && parsed . minute () . is_none () && parsed . second () . is_none () && parsed . subsecond () . is_none () { return Ok (Self :: from_hms_nano (hour , 0 , 0 , 0) ?) ; } match (parsed . minute () , parsed . second () , parsed . subsecond ()) { (None , None , None) => Ok (Self :: from_hms_nano (hour , 0 , 0 , 0) ?) , (Some (minute) , None , None) => Ok (Self :: from_hms_nano (hour , minute , 0 , 0) ?) , (Some (minute) , Some (second) , None) => Ok (Self :: from_hms_nano (hour , minute , second , 0) ?) , (Some (minute) , Some (second) , Some (subsecond)) => { Ok (Self :: from_hms_nano (hour , minute , second , subsecond) ?) } _ => Err (InsufficientInformation) , } } }
};
}
