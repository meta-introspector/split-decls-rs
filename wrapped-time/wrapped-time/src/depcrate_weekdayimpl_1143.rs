// Generated macro for impl_1143 (impl)
macro_rules! Depcrate_weekdayimpl_1143 {
() => {
// Module: crate::weekday
// Provides: {"impl_1143"}
// Dependencies: {}
impl FromStr for Weekday { type Err = error :: InvalidVariant ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "Monday" => Ok (Monday) , "Tuesday" => Ok (Tuesday) , "Wednesday" => Ok (Wednesday) , "Thursday" => Ok (Thursday) , "Friday" => Ok (Friday) , "Saturday" => Ok (Saturday) , "Sunday" => Ok (Sunday) , _ => Err (error :: InvalidVariant) , } } }
};
}
