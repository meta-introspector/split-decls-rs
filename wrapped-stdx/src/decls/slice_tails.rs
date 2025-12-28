macro_rules! slice_tails {
    () => {
        # [doc = " Returns all final segments of the argument, longest first."] pub fn slice_tails < T > (this : & [T]) -> impl Iterator < Item = & [T] > { (0 .. this . len ()) . map (| i | & this [i ..]) }
    };
}

slice_tails!();