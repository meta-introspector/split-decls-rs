// Generated macro for TransitionDay (enum)
macro_rules! Depcrate_data_posixTransitionDay {
() => {
// Module: crate::data::posix
// Provides: {"TransitionDay"}
// Dependencies: {}
# [doc = " A struct for defining a DST transition day."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum TransitionDay { # [doc = " The day of the year, ignoring Feb. 29 on leap years. Designated by `Jn`."] # [doc = ""] # [doc = " Ranges from [1, 365]"] NoLeap (u16) , # [doc = " The day of the year, accounting for Feb. 29 on leap years. Designated by `n`."] # [doc = ""] # [doc = " Ranges from [0, 365]"] WithLeap (u16) , # [doc = " The month, week, day value. Designated by `M.w.d`."] # [doc = ""] # [doc = " `M` ranges from [1, 12]."] # [doc = ""] # [doc = " `w` ranges from [1, 5]."] # [doc = ""] # [doc = " `d` ranges from [0, 6]."] Mwd (u16 , u16 , u16) , }
};
}
