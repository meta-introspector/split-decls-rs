macro_rules! deps {
    () => {
        FseTableMode!();
        FSETable!();
    };
}

macro_rules! choose_table {
    () => {
        deps!();
        fn choose_table < 'a > (previous : Option < & 'a FSETable > , default_table : & 'a FSETable , data : impl Iterator < Item = u8 > , max_log : u8 ,) -> FseTableMode < 'a > { let use_new_table = true ; let use_previous_table = false ; if use_previous_table { FseTableMode :: RepeateLast (previous . unwrap ()) } else if use_new_table { FseTableMode :: Encoded (build_table_from_data (data , max_log , true)) } else { FseTableMode :: Predefined (default_table) } }
    };
}

choose_table!();