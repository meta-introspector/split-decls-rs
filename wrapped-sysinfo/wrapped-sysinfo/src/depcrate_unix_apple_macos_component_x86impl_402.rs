// Generated macro for impl_402 (impl)
macro_rules! Depcrate_unix_apple_macos_component_x86impl_402 {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"impl_402"}
// Dependencies: {}
impl ComponentInner { # [doc = " Creates a new `ComponentInner` with the given information."] pub (crate) fn new (id : String , label : String , max : Option < f32 > , critical : Option < f32 > , key : & [i8] , connection : io_connect_t ,) -> Option < Self > { let ffi_part = ComponentFFI :: new (key , connection) ? ; ffi_part . temperature () . map (| temperature | Self { id , temperature : Some (temperature) , label , max : max . unwrap_or (temperature) , critical , ffi_part , updated : true , }) } pub (crate) fn temperature (& self) -> Option < f32 > { self . temperature } pub (crate) fn max (& self) -> Option < f32 > { Some (self . max) } pub (crate) fn critical (& self) -> Option < f32 > { self . critical } pub (crate) fn label (& self) -> & str { & self . label } pub (crate) fn id (& self) -> Option < & str > { Some (& self . id) } pub (crate) fn refresh (& mut self) { self . temperature = self . ffi_part . temperature () ; if let Some (temperature) = self . temperature { if temperature > self . max { self . max = temperature ; } } } }
};
}
