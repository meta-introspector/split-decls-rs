macro_rules! deps {
    () => {
        Field!();
        Data!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Data < 'a > { pub fn all_fields (& 'a self) -> Box < dyn Iterator < Item = & 'a Field < 'a > > + 'a > { match self { Data :: Enum (variants) => { Box :: new (variants . iter () . flat_map (| variant | variant . fields . iter ())) } Data :: Struct (_ , fields) => Box :: new (fields . iter ()) , } } pub fn has_getter (& self) -> bool { self . all_fields () . any (| f | f . attrs . getter () . is_some ()) } }
    };
}

impl_10!()