macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! build_table_from_data {
    () => {
        deps!();
        pub fn build_table_from_data (data : impl Iterator < Item = u8 > , max_log : u8 , avoid_0_numbit : bool ,) -> FSETable { let mut counts = [0 ; 256] ; let mut max_symbol = 0 ; for x in data { counts [x as usize] += 1 ; } for (idx , count) in counts . iter () . copied () . enumerate () { if count > 0 { max_symbol = idx ; } } build_table_from_counts (& counts [..= max_symbol] , max_log , avoid_0_numbit) }
    };
}

build_table_from_data!()