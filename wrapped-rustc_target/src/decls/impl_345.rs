macro_rules! deps {
    () => {
        OSVersion!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl FromStr for OSVersion { type Err = ParseIntError ; # [doc = " Parse an OS version triple (SDK version or deployment target)."] fn from_str (version : & str) -> Result < Self , ParseIntError > { if let Some ((major , minor)) = version . split_once ('.') { let major = major . parse () ? ; if let Some ((minor , patch)) = minor . split_once ('.') { Ok (Self { major , minor : minor . parse () ? , patch : patch . parse () ? }) } else { Ok (Self { major , minor : minor . parse () ? , patch : 0 }) } } else { Ok (Self { major : version . parse () ? , minor : 0 , patch : 0 }) } } }
    };
}

impl_345!()