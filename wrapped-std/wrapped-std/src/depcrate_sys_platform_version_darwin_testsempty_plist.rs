// Generated macro for empty_plist (function)
macro_rules! Depcrate_sys_platform_version_darwin_testsempty_plist {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"empty_plist"}
// Dependencies: {}
# [test] # [cfg_attr (target_abi = "macabi" , should_panic = "expected iOSSupportVersion in SystemVersion.plist")] # [cfg_attr (not (target_abi = "macabi") , should_panic = "expected ProductVersion in SystemVersion.plist")] fn empty_plist () { let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
        </dict>
        </plist>
    "# ; let cf_handle = CFHandle :: new () ; let _ = parse_version_from_plist (& cf_handle , plist . as_bytes ()) ; }
};
}
