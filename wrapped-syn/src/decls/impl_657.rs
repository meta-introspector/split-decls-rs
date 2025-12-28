macro_rules! deps {
    () => {
        PrivateIterMut!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < 'a , T , P > Iterator for PrivateIterMut < 'a , T , P > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| pair | & mut pair . 0) . or_else (| | self . last . next ()) } }
    };
}

impl_657!()