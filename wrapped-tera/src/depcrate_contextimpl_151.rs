// Generated macro for impl_151 (impl)
macro_rules! Depcrate_contextimpl_151 {
() => {
// Module: crate::context
// Provides: {"impl_151"}
// Dependencies: {}
impl ValueRender for Value { fn render (& self , write : & mut impl Write) -> std :: io :: Result < () > { match * self { Value :: String (ref s) => write ! (write , "{}" , s) , Value :: Number (ref i) => { if let Some (v) = i . as_i64 () { write ! (write , "{}" , v) } else if let Some (v) = i . as_u64 () { write ! (write , "{}" , v) } else if let Some (v) = i . as_f64 () { write ! (write , "{}" , v) } else { unreachable ! () } } Value :: Bool (i) => write ! (write , "{}" , i) , Value :: Null => Ok (()) , Value :: Array (ref a) => { let mut first = true ; write ! (write , "[") ? ; for i in a . iter () { if ! first { write ! (write , ", ") ? ; } first = false ; i . render (write) ? ; } write ! (write , "]") ? ; Ok (()) } Value :: Object (_) => write ! (write , "[object]") , } } }
};
}
