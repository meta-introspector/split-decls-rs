macro_rules! deps {
    () => {
        CacheEntry!();
        SourceFile!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl CacheEntry { # [inline] fn update (& mut self , new_file_and_idx : Option < (Arc < SourceFile > , usize) > , pos : BytePos , time_stamp : usize ,) { if let Some ((file , file_idx)) = new_file_and_idx { self . file = file ; self . file_index = file_idx ; } let pos = self . file . relative_position (pos) ; let line_index = self . file . lookup_line (pos) . unwrap () ; let line_bounds = self . file . line_bounds (line_index) ; self . line_number = line_index + 1 ; self . line = line_bounds ; self . touch (time_stamp) ; } # [inline] fn touch (& mut self , time_stamp : usize) { self . time_stamp = time_stamp ; } }
    };
}

impl_2!();