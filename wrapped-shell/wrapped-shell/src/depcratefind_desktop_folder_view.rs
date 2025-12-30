// Generated macro for find_desktop_folder_view (function)
macro_rules! Depcratefind_desktop_folder_view {
() => {
// Module: crate
// Provides: {"find_desktop_folder_view"}
// Dependencies: {}
fn find_desktop_folder_view < T : Interface > () -> Result < T > { unsafe { let windows : IShellWindows = CoCreateInstance (& ShellWindows , None , CLSCTX_ALL) ? ; let mut handle = 0 ; let desktop = windows . FindWindowSW (& VARIANT :: from (CSIDL_DESKTOP) , & VARIANT :: default () , SWC_DESKTOP , & mut handle , SWFO_NEEDDISPATCH ,) ? ; let provider : IServiceProvider = desktop . cast () ? ; let browser : IShellBrowser = provider . QueryService (& SID_STopLevelBrowser) ? ; let view = browser . QueryActiveShellView () ? ; view . cast () } }
};
}
