macro_rules! deps {
    () => {
        VacantEntry!();
        TableLike!();
        Key!();
        OccupiedEntry!();
        Item!();
        InlineTable!();
        IterMut!();
        Entry!();
        KeyMut!();
        Iter!();
        Value!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl TableLike for InlineTable { fn iter (& self) -> Iter < '_ > { Box :: new (self . items . iter () . map (| (key , value) | (key . get () , value))) } fn iter_mut (& mut self) -> IterMut < '_ > { use indexmap :: map :: MutableKeys ; Box :: new (self . items . iter_mut2 () . map (| (key , value) | (key . as_mut () , value)) ,) } fn clear (& mut self) { self . clear () ; } fn entry < 'a > (& 'a mut self , key : & str) -> crate :: Entry < 'a > { match self . items . entry (key . into ()) { indexmap :: map :: Entry :: Occupied (entry) => { crate :: Entry :: Occupied (crate :: OccupiedEntry { entry }) } indexmap :: map :: Entry :: Vacant (entry) => { crate :: Entry :: Vacant (crate :: VacantEntry { entry }) } } } fn entry_format < 'a > (& 'a mut self , key : & Key) -> crate :: Entry < 'a > { match self . items . entry (key . get () . into ()) { indexmap :: map :: Entry :: Occupied (entry) => { crate :: Entry :: Occupied (crate :: OccupiedEntry { entry }) } indexmap :: map :: Entry :: Vacant (entry) => { crate :: Entry :: Vacant (crate :: VacantEntry { entry }) } } } fn get < 's > (& 's self , key : & str) -> Option < & 's Item > { self . items . get (key) } fn get_mut < 's > (& 's mut self , key : & str) -> Option < & 's mut Item > { self . items . get_mut (key) } fn get_key_value < 'a > (& 'a self , key : & str) -> Option < (& 'a Key , & 'a Item) > { self . get_key_value (key) } fn get_key_value_mut < 'a > (& 'a mut self , key : & str) -> Option < (KeyMut < 'a > , & 'a mut Item) > { self . get_key_value_mut (key) } fn contains_key (& self , key : & str) -> bool { self . contains_key (key) } fn insert (& mut self , key : & str , value : Item) -> Option < Item > { self . insert (key , value . into_value () . unwrap ()) . map (Item :: Value) } fn remove (& mut self , key : & str) -> Option < Item > { self . remove (key) . map (Item :: Value) } fn get_values (& self) -> Vec < (Vec < & Key > , & Value) > { self . get_values () } fn fmt (& mut self) { self . fmt () ; } fn sort_values (& mut self) { self . sort_values () ; } fn set_dotted (& mut self , yes : bool) { self . set_dotted (yes) ; } fn is_dotted (& self) -> bool { self . is_dotted () } fn key (& self , key : & str) -> Option < & '_ Key > { self . key (key) } fn key_mut (& mut self , key : & str) -> Option < KeyMut < '_ > > { self . key_mut (key) } }
    };
}

impl_102!();