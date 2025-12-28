macro_rules! deps {
    () => {
        GrowingHashmapMapElemChar!();
        GrowingHashmapChar!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < ValueType > GrowingHashmapChar < ValueType > where ValueType : Default + Clone + Eq + Copy , { fn get (& self , key : u32) -> ValueType { self . map . as_ref () . map_or_else (| | Default :: default () , | map | map [self . lookup (key)] . value) } fn get_mut (& mut self , key : u32) -> & mut ValueType { if self . map . is_none () { self . allocate () ; } let mut i = self . lookup (key) ; if self . map . as_ref () . expect ("map should have been created above") [i] . value == Default :: default () { self . fill += 1 ; if self . fill * 3 >= (self . mask + 1) * 2 { self . grow ((self . used + 1) * 2) ; i = self . lookup (key) ; } self . used += 1 ; } let elem = & mut self . map . as_mut () . expect ("map should have been created above") [i] ; elem . key = key ; & mut elem . value } fn allocate (& mut self) { self . mask = 8 - 1 ; self . map = Some (vec ! [GrowingHashmapMapElemChar :: default () ; 8]) ; } # [doc = " lookup key inside the hashmap using a similar collision resolution"] # [doc = " strategy to `CPython` and `Ruby`"] fn lookup (& self , key : u32) -> usize { let hash = key ; let mut i = hash as usize & self . mask as usize ; let map = self . map . as_ref () . expect ("callers have to ensure map is allocated") ; if map [i] . value == Default :: default () || map [i] . key == key { return i ; } let mut perturb = key ; loop { i = (i * 5 + perturb as usize + 1) & self . mask as usize ; if map [i] . value == Default :: default () || map [i] . key == key { return i ; } perturb >>= 5 ; } } fn grow (& mut self , min_used : i32) { let mut new_size = self . mask + 1 ; while new_size <= min_used { new_size <<= 1 ; } self . fill = self . used ; self . mask = new_size - 1 ; let old_map = std :: mem :: replace (self . map . as_mut () . expect ("callers have to ensure map is allocated") , vec ! [GrowingHashmapMapElemChar ::< ValueType >:: default () ; new_size as usize] ,) ; for elem in old_map { if elem . value != Default :: default () { let j = self . lookup (elem . key) ; let new_elem = & mut self . map . as_mut () . expect ("map created above") [j] ; new_elem . key = elem . key ; new_elem . value = elem . value ; self . used -= 1 ; if self . used == 0 { break ; } } } self . used = self . fill ; } }
    };
}

impl_23!();