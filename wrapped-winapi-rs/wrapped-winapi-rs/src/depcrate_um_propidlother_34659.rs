// Generated macro for other_34659 (other)
macro_rules! Depcrate_um_propidlother_34659 {
() => {
// Module: crate::um::propidl
// Provides: {"other_34659"}
// Dependencies: {}
extern "system" { pub fn StgConvertVariantToProperty (pvar : * const PROPVARIANT , CodePage : USHORT , pprop : * mut SERIALIZEDPROPERTYVALUE , pcb : * mut ULONG , pid : PROPID , fReserved : BOOLEAN , pcIndirect : * mut ULONG ,) -> * mut SERIALIZEDPROPERTYVALUE ; pub fn StgConvertPropertyToVariant (pprop : * const SERIALIZEDPROPERTYVALUE , CodePage : USHORT , pvar : * mut PROPVARIANT , pma : * mut PMemoryAllocator) -> BOOLEAN ; }
};
}
