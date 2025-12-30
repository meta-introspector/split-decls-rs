// Generated macro for impl_8 (impl)
macro_rules! Depcrate_paramsimpl_8 {
() => {
// Module: crate::params
// Provides: {"impl_8"}
// Dependencies: {}
impl Params { # [doc = " Returns the number of parameters."] # [inline] pub fn len (& self) -> usize { self . len } # [doc = " Returns `true` if there are no parameters present."] # [inline] pub fn is_empty (& self) -> bool { self . len == 0 } # [doc = " Returns an iterator over all parameters and subparameters."] # [inline] pub fn iter (& self) -> ParamsIter < '_ > { ParamsIter :: new (self) } # [doc = " Returns `true` if there is no more space for additional parameters."] # [inline] pub (crate) fn is_full (& self) -> bool { self . len == MAX_PARAMS } # [doc = " Clear all parameters."] # [inline] pub (crate) fn clear (& mut self) { self . current_subparams = 0 ; self . len = 0 ; } # [doc = " Add an additional parameter."] # [inline] pub (crate) fn push (& mut self , item : u16) { self . subparams [self . len - self . current_subparams as usize] = self . current_subparams + 1 ; self . params [self . len] = item ; self . current_subparams = 0 ; self . len += 1 ; } # [doc = " Add an additional subparameter to the current parameter."] # [inline] pub (crate) fn extend (& mut self , item : u16) { self . subparams [self . len - self . current_subparams as usize] = self . current_subparams + 1 ; self . params [self . len] = item ; self . current_subparams += 1 ; self . len += 1 ; } }
};
}
