// Generated macro for impl_4 (impl)
macro_rules! Depcrate_listimpl_4 {
() => {
// Module: crate::list
// Provides: {"impl_4"}
// Dependencies: {}
impl List { fn list_offset_for (& self , entry_in_view : Option < usize > , height : usize) -> usize { match entry_in_view { Some (pos) => match height as usize { h if (self . offset + h) . saturating_sub (1) < pos => pos - h + 1 , _ if self . offset > pos => pos , _ => self . offset , } , None => 0 , } } }
};
}
