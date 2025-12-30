// Generated macro for dt_unit (macro)
macro_rules! Depcrate_typesdt_unit {
() => {
// Module: crate::types
// Provides: {"dt_unit"}
// Dependencies: {}
# [doc = " This macro defines a struct for 0-based date fields: hours, minutes, seconds"] # [doc = " and fractional seconds. Each unit is bounded by a range. The traits implemented"] # [doc = " here will return a Result on whether or not the unit is in range from the given"] # [doc = " input."] macro_rules ! dt_unit { ($ name : ident , $ storage : ident , $ value : expr , $ (# [$ docs : meta]) +) => { $ (# [$ docs]) + # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct $ name ($ storage) ; impl $ name { # [doc = " Gets the numeric value for this component."] pub const fn number (self) -> $ storage { self . 0 } # [doc = " Creates a new value at 0."] pub const fn zero () -> $ name { Self (0) } # [doc = " Returns whether the value is zero."] # [inline] pub fn is_zero (self) -> bool { self . 0 == 0 } } impl TryFrom <$ storage > for $ name { type Error = RangeError ; fn try_from (input : $ storage) -> Result < Self , Self :: Error > { if input > $ value { Err (RangeError { field : stringify ! ($ name) , min : 0 , max : $ value , value : input as i32 , }) } else { Ok (Self (input)) } } } impl TryFrom < usize > for $ name { type Error = RangeError ; fn try_from (input : usize) -> Result < Self , Self :: Error > { if input > $ value { Err (RangeError { field : "$name" , min : 0 , max : $ value , value : input as i32 , }) } else { Ok (Self (input as $ storage)) } } } impl From <$ name > for $ storage { fn from (input : $ name) -> Self { input . 0 } } impl From <$ name > for usize { fn from (input : $ name) -> Self { input . 0 as Self } } } ; }
};
}
