macro_rules! deps {
    () => {
        CompressionLevel!();
        MatchGeneratorDriver!();
        Sequence!();
        Matcher!();
        SuffixStore!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl Matcher for MatchGeneratorDriver { fn reset (& mut self , _level : CompressionLevel) { let vec_pool = & mut self . vec_pool ; let suffix_pool = & mut self . suffix_pool ; self . match_generator . reset (| mut data , mut suffixes | { data . resize (data . capacity () , 0) ; vec_pool . push (data) ; suffixes . slots . clear () ; suffixes . slots . resize (suffixes . slots . capacity () , None) ; suffix_pool . push (suffixes) ; }) ; } fn window_size (& self) -> u64 { self . match_generator . max_window_size as u64 } fn get_next_space (& mut self) -> Vec < u8 > { self . vec_pool . pop () . unwrap_or_else (| | { let mut space = alloc :: vec ! [0 ; self . slice_size] ; space . resize (space . capacity () , 0) ; space }) } fn get_last_space (& mut self) -> & [u8] { self . match_generator . window . last () . unwrap () . data . as_slice () } fn commit_space (& mut self , space : Vec < u8 >) { let vec_pool = & mut self . vec_pool ; let suffixes = self . suffix_pool . pop () . unwrap_or_else (| | SuffixStore :: with_capacity (space . len ())) ; let suffix_pool = & mut self . suffix_pool ; self . match_generator . add_data (space , suffixes , | mut data , mut suffixes | { data . resize (data . capacity () , 0) ; vec_pool . push (data) ; suffixes . slots . clear () ; suffixes . slots . resize (suffixes . slots . capacity () , None) ; suffix_pool . push (suffixes) ; }) ; } fn start_matching (& mut self , mut handle_sequence : impl for < 'a > FnMut (Sequence < 'a >)) { while self . match_generator . next_sequence (& mut handle_sequence) { } } fn skip_matching (& mut self) { self . match_generator . skip_matching () ; } }
    };
}

impl_250!()