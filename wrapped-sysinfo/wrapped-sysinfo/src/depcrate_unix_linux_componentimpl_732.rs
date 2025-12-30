// Generated macro for impl_732 (impl)
macro_rules! Depcrate_unix_linux_componentimpl_732 {
() => {
// Module: crate::unix::linux::component
// Provides: {"impl_732"}
// Dependencies: {}
impl ComponentInner { fn update_from (& mut self , Component { inner : ComponentInner { temperature , max , input_file , highest_file , .. } , } : Component ,) { if let Some (temp) = temperature { self . temperature = Some (temp) ; } match (max , self . max) { (Some (new_max) , Some (old_max)) => self . max = Some (new_max . max (old_max)) , (Some (max) , None) => self . max = Some (max) , _ => { } } if input_file . is_some () && input_file != self . input_file { self . input_file = input_file ; } if highest_file . is_some () && highest_file != self . highest_file { self . highest_file = highest_file ; } self . updated = true ; } }
};
}
