macro_rules! deps {
    () => {
        Active!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl Active { fn next_value (& mut self) -> u32 { self . prev_value += 1 ; self . prev_value } fn get (& self , key : usize) -> Option < u32 > { self . map . get (& key) . copied () } fn get_any (& self , seed : usize) -> Option < (usize , u32) > { if self . map . is_empty () { return None ; } let index = seed % self . map . len () ; self . map . get_index (index) . map (| (k , v) | (* k , * v)) } fn insert (& mut self , key : usize , value : u32) { assert_eq ! (self . map . insert (key , value) , None , "keys of active entries must be unique") ; } fn remove (& mut self , key : usize) -> Option < u32 > { self . map . swap_remove (& key) } fn remove_any (& mut self , seed : usize) -> Option < (usize , u32) > { if self . map . is_empty () { return None ; } let index = seed % self . map . len () ; self . map . swap_remove_index (index) } fn drain (& mut self) -> impl Iterator < Item = (usize , u32) > + '_ { self . map . drain (..) } }
    };
}

impl_280!();