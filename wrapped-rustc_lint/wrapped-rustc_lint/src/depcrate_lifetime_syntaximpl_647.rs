// Generated macro for impl_647 (impl)
macro_rules! Depcrate_lifetime_syntaximpl_647 {
() => {
// Module: crate::lifetime_syntax
// Provides: {"impl_647"}
// Dependencies: {}
impl < T > LifetimeSyntaxCategories < T > { fn select (& mut self , category : LifetimeSyntaxCategory) -> & mut T { use LifetimeSyntaxCategory :: * ; match category { Elided => & mut self . elided , Hidden => & mut self . hidden , Named => & mut self . named , } } }
};
}
