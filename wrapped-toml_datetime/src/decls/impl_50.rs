macro_rules! deps {
    () => {
        DatetimeOrTable!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: Visitor < 'de > for DatetimeOrTable < '_ , 'de > { type Value = () ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { formatter . write_str ("a string key") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : serde_core :: de :: Error , { if s == crate :: datetime :: FIELD { * self . key = None ; Ok (()) } else { use crate :: alloc :: borrow :: ToOwned as _ ; * self . key = Some (alloc :: borrow :: Cow :: Owned (s . to_owned ())) ; Ok (()) } } fn visit_borrowed_str < E > (self , s : & 'de str) -> Result < Self :: Value , E > where E : serde_core :: de :: Error , { if s == crate :: datetime :: FIELD { * self . key = None ; Ok (()) } else { * self . key = Some (alloc :: borrow :: Cow :: Borrowed (s)) ; Ok (()) } } # [allow (unused_qualifications)] fn visit_string < E > (self , s : alloc :: string :: String) -> Result < Self :: Value , E > where E : serde_core :: de :: Error , { if s == crate :: datetime :: FIELD { * self . key = None ; Ok (()) } else { * self . key = Some (alloc :: borrow :: Cow :: Owned (s)) ; Ok (()) } } }
    };
}

impl_50!();