macro_rules! deps {
    () => {
        CollectAndApply!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        # [doc = " The blanket impl that always collects all elements and applies `f`."] impl < T , R > CollectAndApply < T , R > for T { type Output = R ; # [doc = " Equivalent to `f(&iter.collect::<Vec<_>>())`."] fn collect_and_apply < I , F > (mut iter : I , f : F) -> R where I : Iterator < Item = T > , F : FnOnce (& [T]) -> R , { let Some (t0) = iter . next () else { return f (& []) ; } ; let Some (t1) = iter . next () else { return f (& [t0]) ; } ; let Some (t2) = iter . next () else { return f (& [t0 , t1]) ; } ; let Some (t3) = iter . next () else { return f (& [t0 , t1 , t2]) ; } ; let Some (t4) = iter . next () else { return f (& [t0 , t1 , t2 , t3]) ; } ; let Some (t5) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4]) ; } ; let Some (t6) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4 , t5]) ; } ; let Some (t7) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6]) ; } ; let Some (t8) = iter . next () else { return f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6 , t7]) ; } ; f (& [t0 , t1 , t2 , t3 , t4 , t5 , t6 , t7 , t8] . into_iter () . chain (iter) . collect :: < Vec < _ > > ()) } }
    };
}

impl_329!();