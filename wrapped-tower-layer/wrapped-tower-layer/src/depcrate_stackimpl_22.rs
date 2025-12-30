// Generated macro for impl_22 (impl)
macro_rules! Depcrate_stackimpl_22 {
() => {
// Module: crate::stack
// Provides: {"impl_22"}
// Dependencies: {}
impl < S , Inner , Outer > Layer < S > for Stack < Inner , Outer > where Inner : Layer < S > , Outer : Layer < Inner :: Service > , { type Service = Outer :: Service ; fn layer (& self , service : S) -> Self :: Service { let inner = self . inner . layer (service) ; self . outer . layer (inner) } }
};
}
