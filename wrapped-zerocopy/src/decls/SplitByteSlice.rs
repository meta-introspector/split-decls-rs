macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! SplitByteSlice {
    () => {
        deps!();
        # [doc = " A [`ByteSlice`] that can be split in two."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Unsafe code may depend for its soundness on the assumption that `split_at`"] # [doc = " and `split_at_unchecked` are implemented correctly. In particular, given `B:"] # [doc = " SplitByteSlice` and `b: B`, if `b.deref()` returns a byte slice with address"] # [doc = " `addr` and length `len`, then if `split <= len`, both of these"] # [doc = " invocations:"] # [doc = " - `b.split_at(split)`"] # [doc = " - `b.split_at_unchecked(split)`"] # [doc = ""] # [doc = " ...will return `(first, second)` such that:"] # [doc = " - `first`'s address is `addr` and its length is `split`"] # [doc = " - `second`'s address is `addr + split` and its length is `len - split`"] pub unsafe trait SplitByteSlice : ByteSlice { # [doc = " Attempts to split `self` at the midpoint."] # [doc = ""] # [doc = " `s.split_at(mid)` returns `Ok((s[..mid], s[mid..]))` if `mid <="] # [doc = " s.deref().len()` and otherwise returns `Err(s)`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Unsafe code may rely on this function correctly implementing the above"] # [doc = " functionality."] # [inline] fn split_at (self , mid : usize) -> Result < (Self , Self) , Self > { if mid <= self . deref () . len () { unsafe { Ok (self . split_at_unchecked (mid)) } } else { Err (self) } } # [doc = " Splits the slice at the midpoint, possibly omitting bounds checks."] # [doc = ""] # [doc = " `s.split_at_unchecked(mid)` returns `s[..mid]` and `s[mid..]`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `mid` must not be greater than `self.deref().len()`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Implementations of this method may choose to perform a bounds check and"] # [doc = " panic if `mid > self.deref().len()`. They may also panic for any other"] # [doc = " reason. Since it is optional, callers must not rely on this behavior for"] # [doc = " soundness."] # [must_use] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) ; }
    };
}

SplitByteSlice!();