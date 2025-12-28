macro_rules! deps {
    () => {
        ToStableHashKey!();
        StableCompare!();
        UnordItems!();
    };
}

macro_rules! impl_642 {
    () => {
        deps!();
        impl < T , I : Iterator < Item = T > > UnordItems < T , I > { # [inline] pub fn into_sorted < HCX > (self , hcx : & HCX) -> Vec < T > where T : ToStableHashKey < HCX > , { self . collect_sorted (hcx , true) } # [inline] pub fn into_sorted_stable_ord (self) -> Vec < T > where T : StableCompare , { self . collect_stable_ord_by_key (| x | x) } # [inline] pub fn into_sorted_stable_ord_by_key < K , C > (self , project_to_key : C) -> Vec < T > where K : StableCompare , C : for < 'a > Fn (& 'a T) -> & 'a K , { self . collect_stable_ord_by_key (project_to_key) } # [inline] pub fn collect_sorted < HCX , C > (self , hcx : & HCX , cache_sort_key : bool) -> C where T : ToStableHashKey < HCX > , C : FromIterator < T > + BorrowMut < [T] > , { let mut items : C = self . 0 . collect () ; let slice = items . borrow_mut () ; if slice . len () > 1 { if cache_sort_key { slice . sort_by_cached_key (| x | x . to_stable_hash_key (hcx)) ; } else { slice . sort_by_key (| x | x . to_stable_hash_key (hcx)) ; } } items } # [inline] pub fn collect_stable_ord_by_key < K , C , P > (self , project_to_key : P) -> C where K : StableCompare , P : for < 'a > Fn (& 'a T) -> & 'a K , C : FromIterator < T > + BorrowMut < [T] > , { let mut items : C = self . 0 . collect () ; let slice = items . borrow_mut () ; if slice . len () > 1 { if ! K :: CAN_USE_UNSTABLE_SORT { slice . sort_by (| a , b | { let a_key = project_to_key (a) ; let b_key = project_to_key (b) ; a_key . stable_cmp (b_key) }) ; } else { slice . sort_unstable_by (| a , b | { let a_key = project_to_key (a) ; let b_key = project_to_key (b) ; a_key . stable_cmp (b_key) }) ; } } items } }
    };
}

impl_642!()