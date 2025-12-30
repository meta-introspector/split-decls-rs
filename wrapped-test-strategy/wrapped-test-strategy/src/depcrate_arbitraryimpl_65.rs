// Generated macro for impl_65 (impl)
macro_rules! Depcrate_arbitraryimpl_65 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_65"}
// Dependencies: {}
impl StrategyValueType { fn get (& self) -> Type { match self { StrategyValueType :: Type (ty) => ty . clone () , StrategyValueType :: Map (_) => parse_quote ! (_) , } } fn any (& self) -> TokenStream { match self { StrategyValueType :: Type (ty) => quote ! (proptest :: arbitrary :: any ::<# ty > ()) , StrategyValueType :: Map (expr) => quote ! (_map_to_any (# expr)) , } } fn any_with_args_let (& self , var : TokenStream , init : TokenStream) -> TokenStream { match self { StrategyValueType :: Type (ty) => { quote ! (let mut # var :<# ty as proptest :: arbitrary :: Arbitrary >:: Parameters = # init ;) } StrategyValueType :: Map (expr) => { quote ! (let mut # var = _map_to_any_with_init (# expr , # init) ;) } } } fn any_with (& self , args : TokenStream) -> TokenStream { match self { StrategyValueType :: Type (ty) => quote ! (proptest :: arbitrary :: any_with ::<# ty > (# args)) , StrategyValueType :: Map (expr) => quote ! (_map_to_any_with (# expr , # args)) , } } }
};
}
