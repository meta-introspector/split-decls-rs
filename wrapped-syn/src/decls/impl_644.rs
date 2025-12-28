macro_rules! deps {
    () => {
        PrivateIter!();
    };
}

macro_rules! impl_644 {
    () => {
        deps!();
        impl < 'a , T , P > Iterator for PrivateIter < 'a , T , P > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| pair | & pair . 0) . or_else (| | self . last . next ()) } }
    };
}

impl_644!();