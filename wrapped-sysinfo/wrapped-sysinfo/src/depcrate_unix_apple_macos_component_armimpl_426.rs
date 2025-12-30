// Generated macro for impl_426 (impl)
macro_rules! Depcrate_unix_apple_macos_component_armimpl_426 {
() => {
// Module: crate::unix::apple::macos::component::arm
// Provides: {"impl_426"}
// Dependencies: {}
impl ComponentInner { pub (crate) fn new (id : Option < String > , label : String , max : Option < f32 > , critical : Option < f32 > , service : CFRetained < IOHIDServiceClient > ,) -> Self { Self { id , service , label , max : max . unwrap_or (0.) , critical , temperature : None , updated : true , } } pub (crate) fn temperature (& self) -> Option < f32 > { self . temperature } pub (crate) fn max (& self) -> Option < f32 > { Some (self . max) } pub (crate) fn critical (& self) -> Option < f32 > { self . critical } pub (crate) fn label (& self) -> & str { & self . label } pub (crate) fn id (& self) -> Option < & str > { self . id . as_deref () } pub (crate) fn refresh (& mut self) { unsafe { let Some (event) = IOHIDServiceClientCopyEvent (& self . service , kIOHIDEventTypeTemperature , 0 , 0) else { self . temperature = None ; return ; } ; let event = CFRetained :: from_raw (event) ; let temperature = IOHIDEventGetFloatValue (& event , IOHIDEventFieldBase (kIOHIDEventTypeTemperature)) as _ ; self . temperature = Some (temperature) ; if temperature > self . max { self . max = temperature ; } } } }
};
}
