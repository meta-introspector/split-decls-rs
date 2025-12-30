// Generated macro for other_23955 (other)
macro_rules! Depcrate_um_d2d1_1other_23955 {
() => {
// Module: crate::um::d2d1_1
// Provides: {"other_23955"}
// Dependencies: {}
extern "system" { pub fn D2D1CreateDevice (dxgiDevice : * const IDXGIDevice , creationProperties : * const D2D1_CREATION_PROPERTIES , d2dDevice : * mut * mut ID2D1Device ,) -> HRESULT ; pub fn D2D1CreateDeviceContext (dxgiSurface : * const IDXGISurface , creationProperties : * const D2D1_CREATION_PROPERTIES , d2dDeviceContext : * mut * mut ID2D1DeviceContext ,) -> HRESULT ; pub fn D2D1ConvertColorSpace (sourceColorSpace : D2D1_COLOR_SPACE , destinationColorSpace : D2D1_COLOR_SPACE , color : * const D2D1_COLOR_F ,) -> D2D1_COLOR_F ; pub fn D2D1SinCos (angle : FLOAT , s : * mut FLOAT , c : * mut FLOAT ,) -> () ; pub fn D2D1Tan (angle : FLOAT ,) -> FLOAT ; pub fn D2D1Vec3Length (x : FLOAT , y : FLOAT , z : FLOAT ,) -> FLOAT ; }
};
}
