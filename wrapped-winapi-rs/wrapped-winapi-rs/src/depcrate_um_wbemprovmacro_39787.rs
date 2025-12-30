// Generated macro for macro_39787 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39787 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39787"}
// Dependencies: {}
RIDL ! { # [uuid (0x86336d20 , 0xca11 , 0x4786 , 0x9e , 0xf1 , 0xbc , 0x8a , 0x94 , 0x6b , 0x42 , 0xfc)] interface IWbemDecoupledBasicEventProvider (IWbemDecoupledBasicEventProviderVtbl) : IWbemDecoupledRegistrar (IWbemDecoupledRegistrarVtbl) { fn GetSink (a_Flags : c_long , a_Context : * mut IWbemContext , a_Sink : * mut * mut IWbemObjectSink ,) -> HRESULT , fn GetService (a_Flags : c_long , a_Context : * mut IWbemContext , a_Service : * mut * mut IWbemServices ,) -> HRESULT , } }
};
}
