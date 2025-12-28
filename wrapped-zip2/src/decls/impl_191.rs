macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [cfg (fuzzing)] impl arbitrary :: Arbitrary < '_ > for DateTime { fn arbitrary (u : & mut arbitrary :: Unstructured) -> arbitrary :: Result < Self > { let year : u16 = u . int_in_range (1980 ..= 2107) ? ; let month : u16 = u . int_in_range (1 ..= 12) ? ; let day : u16 = u . int_in_range (1 ..= 31) ? ; let datepart = day | (month << 5) | ((year - 1980) << 9) ; let hour : u16 = u . int_in_range (0 ..= 23) ? ; let minute : u16 = u . int_in_range (0 ..= 59) ? ; let second : u16 = u . int_in_range (0 ..= 58) ? ; let timepart = (second >> 1) | (minute << 5) | (hour << 11) ; Ok (DateTime { datepart , timepart }) } }
    };
}

impl_191!()