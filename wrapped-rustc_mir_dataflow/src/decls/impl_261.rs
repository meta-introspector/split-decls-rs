macro_rules! deps {
    () => {
        Children!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl Iterator for Children < '_ , '_ > { type Item = PlaceIndex ; fn next (& mut self) -> Option < Self :: Item > { match self . next { Some (child) => { self . next = self . map . places [child] . next_sibling ; Some (child) } None => None , } } }
    };
}

impl_261!();