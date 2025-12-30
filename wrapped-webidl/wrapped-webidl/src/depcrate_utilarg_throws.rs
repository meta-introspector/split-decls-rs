// Generated macro for arg_throws (function)
macro_rules! Depcrate_utilarg_throws {
() => {
// Module: crate::util
// Provides: {"arg_throws"}
// Dependencies: {}
fn arg_throws (ty : & IdlType < '_ >) -> bool { match ty { IdlType :: DataView { allow_shared } | IdlType :: Int8Array { allow_shared , .. } | IdlType :: Uint8Array { allow_shared , .. } | IdlType :: Uint8ClampedArray { allow_shared , .. } | IdlType :: Int16Array { allow_shared , .. } | IdlType :: Uint16Array { allow_shared , .. } | IdlType :: Int32Array { allow_shared , .. } | IdlType :: Uint32Array { allow_shared , .. } | IdlType :: Float32Array { allow_shared , .. } | IdlType :: Float64Array { allow_shared , .. } | IdlType :: ArrayBufferView { allow_shared , .. } | IdlType :: BufferSource { allow_shared , .. } | IdlType :: Identifier { ty : IdentifierType :: Int8Slice { allow_shared , .. } | IdentifierType :: Uint8Slice { allow_shared , .. } | IdentifierType :: Uint8ClampedSlice { allow_shared , .. } | IdentifierType :: Int16Slice { allow_shared , .. } | IdentifierType :: Uint16Slice { allow_shared , .. } | IdentifierType :: Int32Slice { allow_shared , .. } | IdentifierType :: Uint32Slice { allow_shared , .. } | IdentifierType :: Float32Slice { allow_shared , .. } | IdentifierType :: Float64Slice { allow_shared , .. } , .. } => ! allow_shared , IdlType :: Nullable (item) => arg_throws (item) , IdlType :: Union (list) => list . iter () . any (arg_throws) , _ => false , } }
};
}
