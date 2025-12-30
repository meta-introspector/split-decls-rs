// Generated macro for impl_11 (impl)
macro_rules! Depcrate_mapimpl_11 {
() => {
// Module: crate::map
// Provides: {"impl_11"}
// Dependencies: {}
impl < T , const N : usize > UnarrayArrayExt < T , N > for [T ; N] { fn map_result < S , E > (self , mut f : impl FnMut (T) -> Result < S , E >) -> Result < [S ; N] , E > { let mut result = uninit_buf () ; for (index , (item , slot)) in IntoIterator :: into_iter (self) . zip (& mut result) . enumerate () { match f (item) { Ok (s) => slot . write (s) , Err (e) => { result . iter_mut () . take (index) . for_each (| slot | unsafe { slot . assume_init_drop () }) ; return Err (e) ; } } ; } Ok (unsafe { mark_initialized (result) }) } fn map_option < S > (self , mut f : impl FnMut (T) -> Option < S >) -> Option < [S ; N] > { let actual_f = | t : T | -> Result < S , () > { f (t) . ok_or (()) } ; let result : Result < [S ; N] , () > = UnarrayArrayExt :: map_result (self , actual_f) ; match result { Ok (result) => Some (result) , Err (()) => None , } } }
};
}
