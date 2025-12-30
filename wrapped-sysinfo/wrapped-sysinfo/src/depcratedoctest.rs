// Generated macro for doctest (module)
macro_rules! Depcratedoctest {
() => {
// Module: crate
// Provides: {"doctest"}
// Dependencies: {}
# [cfg (doctest)] mod doctest { macro_rules ! compile_fail_import { ($ mod_name : ident => $ ($ imports : ident) ,+ $ (,) ?) => { $ (# [doc = concat ! (r"```compile_fail
use sysinfo::" , stringify ! ($ imports) , r";
```
")]) + mod $ mod_name { } } ; } # [cfg (not (feature = "system"))] compile_fail_import ! (no_system_feature => get_current_pid , CGroupLimits , Cpu , CpuRefreshKind , DiskUsage , KillError , LoadAvg , MemoryRefreshKind , Motherboard , Pid , Process , ProcessesToUpdate , ProcessRefreshKind , ProcessStatus , Product , RefreshKind , Signal , System , ThreadKind , UpdateKind ,) ; # [cfg (not (feature = "disk"))] compile_fail_import ! (no_disk_feature => Disk , Disks , DiskKind ,) ; # [cfg (not (feature = "component"))] compile_fail_import ! (no_component_feature => Component , Components ,) ; # [cfg (not (feature = "network"))] compile_fail_import ! (no_network_feature => IpNetwork , MacAddr , NetworkData , Networks ,) ; # [cfg (not (feature = "user"))] compile_fail_import ! (no_user_feature => Group , Groups , User , Users ,) ; }
};
}
