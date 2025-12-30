// Generated macro for flag_slices_allow_shared (function)
macro_rules! Depcrate_utilflag_slices_allow_shared {
() => {
// Module: crate::util
// Provides: {"flag_slices_allow_shared"}
// Dependencies: {}
fn flag_slices_allow_shared (ty : & mut IdlType) { match ty { IdlType :: DataView { allow_shared } | IdlType :: Int8Array { allow_shared , .. } | IdlType :: Uint8Array { allow_shared , .. } | IdlType :: Uint8ClampedArray { allow_shared , .. } | IdlType :: Int16Array { allow_shared , .. } | IdlType :: Uint16Array { allow_shared , .. } | IdlType :: Int32Array { allow_shared , .. } | IdlType :: Uint32Array { allow_shared , .. } | IdlType :: Float32Array { allow_shared , .. } | IdlType :: Float64Array { allow_shared , .. } | IdlType :: ArrayBufferView { allow_shared , .. } | IdlType :: BufferSource { allow_shared , .. } => * allow_shared = true , IdlType :: Nullable (item) => flag_slices_allow_shared (item) , IdlType :: FrozenArray (item) => flag_slices_allow_shared (item) , IdlType :: Sequence (item) => flag_slices_allow_shared (item) , IdlType :: ObservableArray (item) => flag_slices_allow_shared (item) , IdlType :: Promise (item) => flag_slices_allow_shared (item) , IdlType :: Record (item1 , item2) => { flag_slices_allow_shared (item1) ; flag_slices_allow_shared (item2) ; } IdlType :: Union (list) => { for item in list { flag_slices_allow_shared (item) ; } } _ => { } } }
};
}
