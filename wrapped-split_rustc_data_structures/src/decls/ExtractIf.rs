macro_rules! ExtractIf {
    () => {
        # [doc = " An iterator for [`ThinVec`] which uses a closure to determine if an element should be removed."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct ExtractIf < 'a , T , F > { vec : & 'a mut ThinVec < T > , # [doc = " The index of the item that will be inspected by the next call to `next`."] idx : usize , # [doc = " The number of items that have been drained (removed) thus far."] del : usize , # [doc = " The original length of `vec` prior to draining."] old_len : usize , # [doc = " The filter test predicate."] pred : F , }
    };
}

ExtractIf!();