macro_rules! deps {
    () => {
        Mode!();
        Flags!();
        InflateStream!();
        Table!();
        ReturnCode!();
    };
}

macro_rules! reset_keep {
    () => {
        deps!();
        pub fn reset_keep (stream : & mut InflateStream) -> ReturnCode { stream . total_in = 0 ; stream . total_out = 0 ; stream . state . total = 0 ; stream . msg = core :: ptr :: null_mut () ; let state = & mut stream . state ; if state . wrap != 0 { stream . adler = (state . wrap & 1) as _ ; } state . mode = Mode :: Head ; state . checksum = crate :: ADLER32_INITIAL_VALUE as u32 ; state . flags . update (Flags :: IS_LAST_BLOCK , false) ; state . flags . update (Flags :: HAVE_DICT , false) ; state . flags . update (Flags :: SANE , true) ; state . gzip_flags = - 1 ; state . dmax = 32768 ; state . head = None ; state . bit_reader = BitReader :: new (& []) ; state . next = 0 ; state . len_table = Table :: default () ; state . dist_table = Table :: default () ; state . back = usize :: MAX ; ReturnCode :: Ok }
    };
}

reset_keep!();