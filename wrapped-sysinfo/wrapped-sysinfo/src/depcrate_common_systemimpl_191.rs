// Generated macro for impl_191 (impl)
macro_rules! Depcrate_common_systemimpl_191 {
() => {
// Module: crate::common::system
// Provides: {"impl_191"}
// Dependencies: {}
impl ProcessRefreshKind { # [doc = " Creates a new `ProcessRefreshKind` with every refresh set to `false`, except for `tasks`."] # [doc = " By default, we want to list all processes and tasks are considered processes on their own"] # [doc = " in linux so we still fetch them by default. However, the processes information are not"] # [doc = " refreshed."] # [doc = " ```"] # [doc = " use sysinfo::{ProcessRefreshKind, UpdateKind};"] # [doc = ""] # [doc = " let r = ProcessRefreshKind::nothing();"] # [doc = ""] # [doc = " assert_eq!(r.cpu(), false);"] # [doc = " assert_eq!(r.user(), UpdateKind::Never);"] # [doc = " ```"] pub fn nothing () -> Self { Self :: default () } # [doc = " Creates a new `ProcessRefreshKind` with every refresh set to `true` or"] # [doc = " [`UpdateKind::OnlyIfNotSet`]."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::{ProcessRefreshKind, UpdateKind};"] # [doc = ""] # [doc = " let r = ProcessRefreshKind::everything();"] # [doc = ""] # [doc = " assert_eq!(r.cpu(), true);"] # [doc = " assert_eq!(r.user(), UpdateKind::OnlyIfNotSet);"] # [doc = " ```"] pub fn everything () -> Self { Self { cpu : true , disk_usage : true , memory : true , user : UpdateKind :: OnlyIfNotSet , cwd : UpdateKind :: OnlyIfNotSet , root : UpdateKind :: OnlyIfNotSet , environ : UpdateKind :: OnlyIfNotSet , cmd : UpdateKind :: OnlyIfNotSet , exe : UpdateKind :: OnlyIfNotSet , tasks : true , } } impl_get_set ! (ProcessRefreshKind , cpu , with_cpu , without_cpu , "\
It will retrieve both CPU usage and CPU accumulated time,") ; impl_get_set ! (ProcessRefreshKind , disk_usage , with_disk_usage , without_disk_usage) ; impl_get_set ! (ProcessRefreshKind , user , with_user , without_user , UpdateKind , "\
It will retrieve the following information:

 * user ID
 * user effective ID (if available on the platform)
 * user group ID (if available on the platform)
 * user effective ID (if available on the platform)") ; impl_get_set ! (ProcessRefreshKind , memory , with_memory , without_memory) ; impl_get_set ! (ProcessRefreshKind , cwd , with_cwd , without_cwd , UpdateKind) ; impl_get_set ! (ProcessRefreshKind , root , with_root , without_root , UpdateKind) ; impl_get_set ! (ProcessRefreshKind , environ , with_environ , without_environ , UpdateKind) ; impl_get_set ! (ProcessRefreshKind , cmd , with_cmd , without_cmd , UpdateKind) ; impl_get_set ! (ProcessRefreshKind , exe , with_exe , without_exe , UpdateKind) ; impl_get_set ! (ProcessRefreshKind , tasks , with_tasks , without_tasks) ; }
};
}
