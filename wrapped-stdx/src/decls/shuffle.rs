macro_rules! shuffle {
    () => {
        pub fn shuffle < T > (slice : & mut [T] , mut rand_index : impl FnMut (usize) -> usize) { let mut remaining = slice . len () - 1 ; while remaining > 0 { let index = rand_index (remaining) ; slice . swap (remaining , index) ; remaining -= 1 ; } }
    };
}

shuffle!();