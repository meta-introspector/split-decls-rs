use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn par_slice < I : DynSend > (items : & mut [I] , guard : & ParallelGuard , for_each : impl Fn (& mut I) + DynSync + DynSend ,) { struct State < 'a , F > { for_each : FromDyn < F > , guard : & 'a ParallelGuard , group : usize , } fn par_rec < I : DynSend , F : Fn (& mut I) + DynSync + DynSend > (items : & mut [I] , state : & State < '_ , F > ,) { if items . len () <= state . group { for item in items { state . guard . run (| | (state . for_each) (item)) ; } } else { let (left , right) = items . split_at_mut (items . len () / 2) ; let mut left = state . for_each . derive (left) ; let mut right = state . for_each . derive (right) ; rustc_thread_pool :: join (move | | par_rec (* left , state) , move | | par_rec (* right , state)) ; } } let state = State { for_each : FromDyn :: from (for_each) , guard , group : std :: cmp :: max (items . len () / 128 , 1) , } ; par_rec (items , & state) }
}