macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! Filter {
    () => {
        deps!();
        struct Filter < 'a > { all_lines : & 'a [& 'a str] , normalization : Normalization , context : Context < 'a > , hide_numbers : usize , other_types : Option < usize > , }
    };
}

Filter!()