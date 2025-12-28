macro_rules! get_index {
    () => {
        # [inline] fn get_index (i : f64 , array : & [Value]) -> usize { if i >= 0.0 { i as usize } else { (array . len () as f64 + i) as usize } }
    };
}

get_index!();