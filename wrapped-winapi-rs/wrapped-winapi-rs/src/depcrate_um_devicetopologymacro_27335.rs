// Generated macro for macro_27335 (macro)
macro_rules! Depcrate_um_devicetopologymacro_27335 {
() => {
// Module: crate::um::devicetopology
// Provides: {"macro_27335"}
// Dependencies: {}
RIDL ! { # [uuid (0x9c2c4058 , 0x23f5 , 0x41de , 0x87 , 0x7a , 0xdf , 0x3a , 0xf2 , 0x36 , 0xa0 , 0x9e)] interface IConnector (IConnectorVtbl) : IUnknown (IUnknownVtbl) { fn GetType (pType : * mut ConnectorType ,) -> HRESULT , fn GetDataFlow (pFlow : * mut DataFlow ,) -> HRESULT , fn ConnectTo (pConnectTo : * mut IConnector ,) -> HRESULT , fn Disconnect () -> HRESULT , fn IsConnected (pbConnected : * mut BOOL ,) -> HRESULT , fn GetConnectedTo (ppConTo : * mut * mut IConnector ,) -> HRESULT , fn GetConnectorIdConnectedTo (ppwstrConnectorId : * mut LPWSTR ,) -> HRESULT , fn GetDeviceIdConnectedTo (ppwstrDeviceId : * mut LPWSTR ,) -> HRESULT , } }
};
}
