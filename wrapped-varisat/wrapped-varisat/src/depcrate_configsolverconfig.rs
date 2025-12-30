// Generated macro for SolverConfig (struct)
macro_rules! Depcrate_configSolverConfig {
() => {
// Module: crate::config
// Provides: {"SolverConfig"}
// Dependencies: {}
# [doc = " Configurable parameters used during solving."] # [derive (DocDefault , ConfigUpdate)] pub struct SolverConfig { # [doc = " Multiplicative decay for the VSIDS decision heuristic."] # [doc = ""] # [doc = " [default: 0.95]  [range: 0.5..1.0]"] pub vsids_decay : f32 , # [doc = " Multiplicative decay for clause activities."] # [doc = ""] # [doc = " [default: 0.999]  [range: 0.5..1.0]"] pub clause_activity_decay : f32 , # [doc = " Number of conflicts between local clause reductions."] # [doc = ""] # [doc = " [default: 15000]  [range: 1..]"] pub reduce_locals_interval : u64 , # [doc = " Number of conflicts between mid clause reductions."] # [doc = ""] # [doc = " [default: 10000]  [range: 1..]"] pub reduce_mids_interval : u64 , # [doc = " Scaling factor for luby sequence based restarts (number of conflicts)."] # [doc = ""] # [doc = " [default: 128]  [range: 1..]"] pub luby_restart_interval_scale : u64 , }
};
}
