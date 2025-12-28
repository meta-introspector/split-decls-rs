macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [cfg (feature = "nightly_slice_partition_dedup")] impl < A : Array > ArrayVec < A > { # [doc = " De-duplicates the vec contents."] # [inline (always)] pub fn dedup (& mut self) where A :: Item : PartialEq , { self . dedup_by (| a , b | a == b) } # [doc = " De-duplicates the vec according to the predicate given."] # [inline (always)] pub fn dedup_by < F > (& mut self , same_bucket : F) where F : FnMut (& mut A :: Item , & mut A :: Item) -> bool , { let len = { let (dedup , _) = self . as_mut_slice () . partition_dedup_by (same_bucket) ; dedup . len () } ; self . truncate (len) ; } # [doc = " De-duplicates the vec according to the key selector given."] # [inline (always)] pub fn dedup_by_key < F , K > (& mut self , mut key : F) where F : FnMut (& mut A :: Item) -> K , K : PartialEq , { self . dedup_by (| a , b | key (a) == key (b)) } }
    };
}

impl_22!();