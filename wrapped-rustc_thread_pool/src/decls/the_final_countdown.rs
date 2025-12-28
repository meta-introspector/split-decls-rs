macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! the_final_countdown {
    () => {
        deps!();
        fn the_final_countdown < 'scope > (s : & Scope < 'scope > , bottom_of_stack : & 'scope i32 , max : & 'scope Mutex < usize > , n : usize ,) { let top_of_stack = 0 ; let p = bottom_of_stack as * const i32 as usize ; let q = & top_of_stack as * const i32 as usize ; let diff = if p > q { p - q } else { q - p } ; let mut data = max . lock () . unwrap () ; * data = Ord :: max (diff , * data) ; if n > 0 { s . spawn (move | s | the_final_countdown (s , bottom_of_stack , max , n - 1)) ; } }
    };
}

the_final_countdown!()