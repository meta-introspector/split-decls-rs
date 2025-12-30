// Generated macro for flag_slices_immutable (function)
macro_rules! Depcrate_utilflag_slices_immutable {
() => {
// Module: crate::util
// Provides: {"flag_slices_immutable"}
// Dependencies: {}
fn flag_slices_immutable (ty : & mut IdlType) { match ty { IdlType :: Int8Array { immutable , .. } | IdlType :: Uint8Array { immutable , .. } | IdlType :: Uint8ClampedArray { immutable , .. } | IdlType :: Int16Array { immutable , .. } | IdlType :: Uint16Array { immutable , .. } | IdlType :: Int32Array { immutable , .. } | IdlType :: Uint32Array { immutable , .. } | IdlType :: Float32Array { immutable , .. } | IdlType :: Float64Array { immutable , .. } | IdlType :: ArrayBufferView { immutable , .. } | IdlType :: BufferSource { immutable , .. } | IdlType :: Identifier { ty : IdentifierType :: AllowSharedBufferSource { immutable } , .. } => * immutable = true , IdlType :: Nullable (item) => flag_slices_immutable (item) , IdlType :: Union (list) => { for item in list { flag_slices_immutable (item) ; } } _ => { } } }
};
}
