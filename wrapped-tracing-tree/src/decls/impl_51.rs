macro_rules! deps {
    () => {
        DifferenceIter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < L : Iterator < Item = T > , R : Iterator < Item = T > , T , U : PartialEq , F : Fn (& T) -> U > Iterator for DifferenceIter < L , R , F > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { loop { let left = self . left . next () ; let right = self . right . next () ? ; if left . as_ref () . map (& self . compare) != Some ((self . compare) (& right)) { return Some (right) ; } } } }
    };
}

impl_51!();