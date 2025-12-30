// Generated macro for impl_190 (impl)
macro_rules! Depcrate_common_systemimpl_190 {
() => {
// Module: crate::common::system
// Provides: {"impl_190"}
// Dependencies: {}
# [doc = " Creates a new `ProcessRefreshKind` with every refresh set to `false`, except for `tasks`."] # [doc = " By default, we want to list all processes and tasks are considered processes on their own"] # [doc = " in linux so we still fetch them by default. However, the processes information are not"] # [doc = " refreshed."] impl Default for ProcessRefreshKind { fn default () -> Self { Self { cpu : false , disk_usage : false , memory : false , user : UpdateKind :: default () , cwd : UpdateKind :: default () , root : UpdateKind :: default () , environ : UpdateKind :: default () , cmd : UpdateKind :: default () , exe : UpdateKind :: default () , tasks : true , } } }
};
}
