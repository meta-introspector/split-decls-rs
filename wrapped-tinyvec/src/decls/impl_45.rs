macro_rules! deps {
    () => {
        Array!();
        ArrayVecIterator!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < A : Array > DoubleEndedIterator for ArrayVecIterator < A > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { let slice = & mut self . data . as_slice_mut () [self . base as usize .. self . tail as usize] ; let item = slice . last_mut () ? ; self . tail -= 1 ; return Some (core :: mem :: take (item)) ; } # [inline] fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { let base = self . base as usize ; let tail = self . tail as usize ; let slice = & mut self . data . as_slice_mut () [base .. tail] ; let n = n . saturating_add (1) ; if let Some (n) = slice . len () . checked_sub (n) { let item = & mut slice [n] ; self . tail = self . base + n as u16 ; return Some (core :: mem :: take (item)) ; } self . tail = self . base ; return None ; } }
    };
}

impl_45!()