macro_rules! deps {
    () => {
        InlineVacantEntry!();
        Value!();
        Item!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'a > InlineVacantEntry < 'a > { # [doc = " Gets a reference to the entry key"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use toml_edit::Table;"] # [doc = ""] # [doc = " let mut map = Table::new();"] # [doc = ""] # [doc = " assert_eq!(\"foo\", map.entry(\"foo\").key());"] # [doc = " ```"] pub fn key (& self) -> & str { self . entry . key () . get () } # [doc = " Sets the value of the entry with the `VacantEntry`'s key,"] # [doc = " and returns a mutable reference to it"] pub fn insert (self , value : Value) -> & 'a mut Value { let entry = self . entry ; let value = Item :: Value (value) ; entry . insert (value) . as_value_mut () . unwrap () } }
    };
}

impl_109!();