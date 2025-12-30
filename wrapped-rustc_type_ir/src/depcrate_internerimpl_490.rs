// Generated macro for impl_490 (impl)
macro_rules! Depcrate_internerimpl_490 {
() => {
// Module: crate::interner
// Provides: {"impl_490"}
// Dependencies: {}
# [doc = " A fallible impl that will fail, without calling `f`, if there are any"] # [doc = " errors during collection."] impl < T , R , E > CollectAndApply < T , R > for Result < T , E > { type Output = Result < R , E > ; # [doc = " Equivalent to `Ok(f(&iter.collect::<Result<Vec<_>>>()?))`."] fn collect_and_apply < I , F > (mut iter : I , f : F) -> Result < R , E > where I : Iterator < Item = Result < T , E > > , F : FnOnce (& [T]) -> R , { let Some (t0) = iter . next () else { return Ok (f (& [])) ; } ; let t0 = t0 ? ; let Some (t1) = iter . next () else { return Ok (f (& [t0])) ; } ; let t1 = t1 ? ; let Some (t2) = iter . next () else { return Ok (f (& [t0 , t1])) ; } ; let t2 = t2 ? ; let Some (t3) = iter . next () else { return Ok (f (& [t0 , t1 , t2])) ; } ; let t3 = t3 ? ; let Some (t4) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3])) ; } ; let t4 = t4 ? ; let Some (t5) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4])) ; } ; let t5 = t5 ? ; let Some (t6) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4 , t5])) ; } ; let t6 = t6 ? ; let Some (t7) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6])) ; } ; let t7 = t7 ? ; let Some (t8) = iter . next () else { return Ok (f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6 , t7])) ; } ; let t8 = t8 ? ; Ok (f (& [Ok (t0) , Ok (t1) , Ok (t2) , Ok (t3) , Ok (t4) , Ok (t5) , Ok (t6) , Ok (t7) , Ok (t8)] . into_iter () . chain (iter) . collect :: < Result < Vec < _ > , _ > > () ?)) } }
};
}
