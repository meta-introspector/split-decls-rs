// Generated macro for ToRepr (trait)
macro_rules! Depcrate_typekindsToRepr {
() => {
// Module: crate::typekinds
// Provides: {"ToRepr"}
// Dependencies: {}
pub trait ToRepr { fn repr (& self , repr : TypeRepr) -> String ; fn c_repr (& self) -> String { self . repr (TypeRepr :: C) } fn rust_repr (& self) -> String { self . repr (TypeRepr :: Rust) } fn llvm_machine_repr (& self) -> String { self . repr (TypeRepr :: LLVMMachine) } fn acle_notation_repr (& self) -> String { self . repr (TypeRepr :: ACLENotation) } fn size (& self) -> String { self . repr (TypeRepr :: Size) } fn size_literal (& self) -> String { self . repr (TypeRepr :: SizeLiteral) } fn type_kind (& self) -> String { self . repr (TypeRepr :: TypeKind) } fn size_in_bytes_log2 (& self) -> String { self . repr (TypeRepr :: SizeInBytesLog2) } }
};
}
