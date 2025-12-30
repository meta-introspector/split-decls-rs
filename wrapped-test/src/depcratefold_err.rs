// Generated macro for fold_err (function)
macro_rules! Depcratefold_err {
() => {
// Module: crate
// Provides: {"fold_err"}
// Dependencies: {}
fn fold_err < T , E > (result : Result < Result < T , E > , Box < dyn Any + Send > > ,) -> Result < T , Box < dyn Any + Send > > where E : Send + 'static , { match result { Ok (Err (e)) => Err (Box :: new (e)) , Ok (Ok (v)) => Ok (v) , Err (e) => Err (e) , } }
};
}
