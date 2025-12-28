macro_rules! deps {
    () => {
        ThreadIdManager!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ThreadIdManager { const fn new () -> Self { Self { free_from : 0 , free_list : None , } } fn alloc (& mut self) -> usize { if let Some (id) = self . free_list . as_mut () . and_then (| heap | heap . pop ()) { id . 0 } else { let id = self . free_from ; self . free_from += 1 ; id } } fn free (& mut self , id : usize) { self . free_list . get_or_insert_with (BinaryHeap :: new) . push (Reverse (id)) ; } }
    };
}

impl_16!();