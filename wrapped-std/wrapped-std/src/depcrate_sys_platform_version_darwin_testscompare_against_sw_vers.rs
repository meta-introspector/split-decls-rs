// Generated macro for compare_against_sw_vers (function)
macro_rules! Depcrate_sys_platform_version_darwin_testscompare_against_sw_vers {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"compare_against_sw_vers"}
// Dependencies: {}
# [test] # [cfg_attr (not (target_os = "macos") , ignore = "`sw_vers` is only available on host macOS")] fn compare_against_sw_vers () { let sw_vers = Command :: new ("sw_vers") . arg ("-productVersion") . output () . unwrap () . stdout ; let sw_vers = String :: from_utf8 (sw_vers) . unwrap () ; let mut sw_vers = sw_vers . trim () . split ('.') ; let major : i32 = sw_vers . next () . unwrap () . parse () . unwrap () ; let minor : i32 = sw_vers . next () . unwrap_or ("0") . parse () . unwrap () ; let subminor : i32 = sw_vers . next () . unwrap_or ("0") . parse () . unwrap () ; assert_eq ! (sw_vers . count () , 0) ; assert_eq ! (lookup_version () . get () , pack_os_version (major as _ , minor as _ , subminor as _)) ; assert_eq ! (__isOSVersionAtLeast (major , minor , subminor) , 1) ; assert_eq ! (__isOSVersionAtLeast (major , minor , (subminor as u32) . saturating_sub (1) as i32) , 1) ; assert_eq ! (__isOSVersionAtLeast (major , (minor as u32) . saturating_sub (1) as i32 , subminor) , 1) ; assert_eq ! (__isOSVersionAtLeast ((major as u32) . saturating_sub (1) as i32 , minor , subminor) , 1) ; assert_eq ! (__isOSVersionAtLeast (major , minor , subminor + 1) , 0) ; assert_eq ! (__isOSVersionAtLeast (major , minor + 1 , subminor) , 0) ; assert_eq ! (__isOSVersionAtLeast (major + 1 , minor , subminor) , 0) ; }
};
}
