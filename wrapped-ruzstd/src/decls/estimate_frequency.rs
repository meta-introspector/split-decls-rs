macro_rules! estimate_frequency {
    () => {
        # [doc = " Computes a best effort guess as to how many times `pattern` occurs within"] # [doc = " `body`. While not 100% accurate, it will be accurate the vast majority of time"] pub fn estimate_frequency (pattern : & [u8] , body : & [u8]) -> usize { assert ! (body . len () >= pattern . len ()) ; const PRIME : isize = 2654435761 ; const ALPHABET_SIZE : isize = 256 ; let mut pattern_hash : isize = 0 ; let mut window_hash : isize = 0 ; let mut h : isize = 1 ; h = (h * ALPHABET_SIZE) % PRIME ; for i in 0 .. pattern . len () { pattern_hash = (ALPHABET_SIZE * pattern_hash + pattern [i] as isize) % PRIME ; window_hash = (ALPHABET_SIZE * window_hash + body [i] as isize) % PRIME ; } let mut num_occurances = 0 ; for i in 0 ..= body . len () - pattern . len () { if pattern_hash == window_hash { num_occurances += 1 ; } if i < body . len () - pattern . len () { window_hash = (ALPHABET_SIZE * (window_hash - body [i] as isize * h) + body [i + pattern . len ()] as isize) % PRIME ; } } num_occurances }
    };
}

estimate_frequency!()