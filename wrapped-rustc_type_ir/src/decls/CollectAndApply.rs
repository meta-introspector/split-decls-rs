macro_rules! CollectAndApply {
    () => {
        # [doc = " Imagine you have a function `F: FnOnce(&[T]) -> R`, plus an iterator `iter`"] # [doc = " that produces `T` items. You could combine them with"] # [doc = " `f(&iter.collect::<Vec<_>>())`, but this requires allocating memory for the"] # [doc = " `Vec`."] # [doc = ""] # [doc = " This trait allows for faster implementations, intended for cases where the"] # [doc = " number of items produced by the iterator is small. There is a blanket impl"] # [doc = " for `T` items, but there is also a fallible impl for `Result<T, E>` items."] pub trait CollectAndApply < T , R > : Sized { type Output ; # [doc = " Produce a result of type `Self::Output` from `iter`. The result will"] # [doc = " typically be produced by applying `f` on the elements produced by"] # [doc = " `iter`, though this may not happen in some impls, e.g. if an error"] # [doc = " occurred during iteration."] fn collect_and_apply < I , F > (iter : I , f : F) -> Self :: Output where I : Iterator < Item = Self > , F : FnOnce (& [T]) -> R ; }
    };
}

CollectAndApply!()