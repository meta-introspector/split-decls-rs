// Generated macro for impl_647 (impl)
macro_rules! Depcrate_follow_redirect_policyimpl_647 {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"impl_647"}
// Dependencies: {}
impl < T > PolicyExt for T where T : ? Sized , { fn and < P , B , E > (self , other : P) -> And < Self , P > where Self : Policy < B , E > + Sized , P : Policy < B , E > , { And :: new (self , other) } fn or < P , B , E > (self , other : P) -> Or < Self , P > where Self : Policy < B , E > + Sized , P : Policy < B , E > , { Or :: new (self , other) } }
};
}
