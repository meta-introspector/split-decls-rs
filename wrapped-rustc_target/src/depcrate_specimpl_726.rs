// Generated macro for impl_726 (impl)
macro_rules! Depcrate_specimpl_726 {
() => {
// Module: crate::spec
// Provides: {"impl_726"}
// Dependencies: {}
impl LinkSelfContainedComponents { # [doc = " Return the component's name."] # [doc = ""] # [doc = " Returns `None` if the bitflags aren't a singular component (but a mix of multiple flags)."] pub fn as_str (self) -> Option < & 'static str > { Some (match self { LinkSelfContainedComponents :: CRT_OBJECTS => "crto" , LinkSelfContainedComponents :: LIBC => "libc" , LinkSelfContainedComponents :: UNWIND => "unwind" , LinkSelfContainedComponents :: LINKER => "linker" , LinkSelfContainedComponents :: SANITIZERS => "sanitizers" , LinkSelfContainedComponents :: MINGW => "mingw" , _ => return None , }) } # [doc = " Returns an array of all the components."] fn all_components () -> [LinkSelfContainedComponents ; 6] { [LinkSelfContainedComponents :: CRT_OBJECTS , LinkSelfContainedComponents :: LIBC , LinkSelfContainedComponents :: UNWIND , LinkSelfContainedComponents :: LINKER , LinkSelfContainedComponents :: SANITIZERS , LinkSelfContainedComponents :: MINGW ,] } # [doc = " Returns whether at least a component is enabled."] pub fn are_any_components_enabled (self) -> bool { ! self . is_empty () } # [doc = " Returns whether `LinkSelfContainedComponents::LINKER` is enabled."] pub fn is_linker_enabled (self) -> bool { self . contains (LinkSelfContainedComponents :: LINKER) } # [doc = " Returns whether `LinkSelfContainedComponents::CRT_OBJECTS` is enabled."] pub fn is_crt_objects_enabled (self) -> bool { self . contains (LinkSelfContainedComponents :: CRT_OBJECTS) } }
};
}
