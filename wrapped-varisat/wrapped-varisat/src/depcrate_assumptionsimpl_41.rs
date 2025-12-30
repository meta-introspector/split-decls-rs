// Generated macro for impl_41 (impl)
macro_rules! Depcrate_assumptionsimpl_41 {
() => {
// Module: crate::assumptions
// Provides: {"impl_41"}
// Dependencies: {}
impl Assumptions { # [doc = " Current number of decision levels used for assumptions."] pub fn assumption_levels (& self) -> usize { self . assumption_levels } # [doc = " Resets assumption_levels to zero on a full restart."] pub fn full_restart (& mut self) { self . assumption_levels = 0 ; } # [doc = " Subset of assumptions that made the formula unsatisfiable."] pub fn failed_core (& self) -> & [Lit] { & self . failed_core } # [doc = " Subset of assumptions that made the formula unsatisfiable."] pub fn user_failed_core (& self) -> & [Lit] { & self . user_failed_core } # [doc = " Current assumptions."] pub fn assumptions (& self) -> & [Lit] { & self . assumptions } }
};
}
