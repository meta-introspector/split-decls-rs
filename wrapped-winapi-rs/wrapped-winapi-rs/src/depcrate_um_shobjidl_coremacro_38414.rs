// Generated macro for macro_38414 (macro)
macro_rules! Depcrate_um_shobjidl_coremacro_38414 {
() => {
// Module: crate::um::shobjidl_core
// Provides: {"macro_38414"}
// Dependencies: {}
RIDL ! { # [uuid (0xb92b56a9 , 0x8b55 , 0x4e14 , 0x9a , 0x89 , 0x01 , 0x99 , 0xbb , 0xb6 , 0xf9 , 0x3b)] interface IDesktopWallpaper (IDesktopWallpaperVtbl) : IUnknown (IUnknownVtbl) { fn SetWallpaper (monitorID : LPCWSTR , wallpaper : LPCWSTR ,) -> HRESULT , fn GetWallpaper (monitorID : LPCWSTR , wallpaper : * mut LPWSTR ,) -> HRESULT , fn GetMonitorDevicePathAt (monitorIndex : UINT , monitorID : * mut LPWSTR ,) -> HRESULT , fn GetMonitorDevicePathCount (count : * mut UINT ,) -> HRESULT , fn GetMonitorRECT (monitorID : LPCWSTR , displayRect : * mut RECT ,) -> HRESULT , fn SetBackgroundColor (color : COLORREF ,) -> HRESULT , fn GetBackgroundColor (color : * mut COLORREF ,) -> HRESULT , fn SetPosition (position : DESKTOP_WALLPAPER_POSITION ,) -> HRESULT , fn GetPosition (position : * mut DESKTOP_WALLPAPER_POSITION ,) -> HRESULT , fn SetSlideshow (items : * mut IShellItemArray ,) -> HRESULT , fn GetSlideshow (items : * mut * mut IShellItemArray ,) -> HRESULT , fn SetSlideshowOptions (options : DESKTOP_SLIDESHOW_OPTIONS , slideshowTick : UINT ,) -> HRESULT , fn GetSlideshowOptions (options : * mut DESKTOP_SLIDESHOW_OPTIONS , slideshowTick : * mut UINT ,) -> HRESULT , fn AdvanceSlideshow (monitorID : LPCWSTR , direction : DESKTOP_SLIDESHOW_DIRECTION ,) -> HRESULT , fn GetStatus (state : * mut DESKTOP_SLIDESHOW_STATE ,) -> HRESULT , fn Enable (enable : BOOL ,) -> HRESULT , } }
};
}
