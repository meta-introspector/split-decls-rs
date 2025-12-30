// Generated macro for impl_32 (impl)
macro_rules! Depcrate_algorithms_replaceimpl_32 {
() => {
// Module: crate::algorithms::replace
// Provides: {"impl_32"}
// Dependencies: {}
impl < D : DiffHook > Replace < D > { # [doc = " Creates a new replace hook wrapping another hook."] pub fn new (d : D) -> Self { Replace { d , del : None , ins : None , eq : None , } } # [doc = " Extracts the inner hook."] pub fn into_inner (self) -> D { self . d } fn flush_eq (& mut self) -> Result < () , D :: Error > { if let Some ((eq_old_index , eq_new_index , eq_len)) = self . eq . take () { self . d . equal (eq_old_index , eq_new_index , eq_len) ? } Ok (()) } fn flush_del_ins (& mut self) -> Result < () , D :: Error > { if let Some ((del_old_index , del_old_len , del_new_index)) = self . del . take () { if let Some ((_ , ins_new_index , ins_new_len)) = self . ins . take () { self . d . replace (del_old_index , del_old_len , ins_new_index , ins_new_len) ? ; } else { self . d . delete (del_old_index , del_old_len , del_new_index) ? ; } } else if let Some ((ins_old_index , ins_new_index , ins_new_len)) = self . ins . take () { self . d . insert (ins_old_index , ins_new_index , ins_new_len) ? ; } Ok (()) } }
};
}
