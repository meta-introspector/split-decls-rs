// Generated macro for impl_607 (impl)
macro_rules! Depcrate_monthimpl_607 {
() => {
// Module: crate::month
// Provides: {"impl_607"}
// Dependencies: {}
impl TryFrom < u8 > for Month { type Error = error :: ComponentRange ; # [inline] fn try_from (value : u8) -> Result < Self , Self :: Error > { match NonZero :: new (value) { Some (value) => Self :: from_number (value) , None => Err (error :: ComponentRange { name : "month" , minimum : 1 , maximum : 12 , value : 0 , conditional_message : None , }) , } } }
};
}
