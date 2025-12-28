macro_rules! deps {
    () => {
        Entry!();
        Key!();
        Item!();
        Value!();
        Table!();
        Iter!();
        IterMut!();
        TableLike!();
        KeyMut!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl TableLike for Table { fn iter (& self) -> Iter < '_ > { self . iter () } fn iter_mut (& mut self) -> IterMut < '_ > { self . iter_mut () } fn clear (& mut self) { self . clear () ; } fn entry < 'a > (& 'a mut self , key : & str) -> Entry < 'a > { self . entry (key) } fn entry_format < 'a > (& 'a mut self , key : & Key) -> Entry < 'a > { self . entry_format (key) } fn get < 's > (& 's self , key : & str) -> Option < & 's Item > { self . get (key) } fn get_mut < 's > (& 's mut self , key : & str) -> Option < & 's mut Item > { self . get_mut (key) } fn get_key_value < 'a > (& 'a self , key : & str) -> Option < (& 'a Key , & 'a Item) > { self . get_key_value (key) } fn get_key_value_mut < 'a > (& 'a mut self , key : & str) -> Option < (KeyMut < 'a > , & 'a mut Item) > { self . get_key_value_mut (key) } fn contains_key (& self , key : & str) -> bool { self . contains_key (key) } fn insert (& mut self , key : & str , value : Item) -> Option < Item > { self . insert (key , value) } fn remove (& mut self , key : & str) -> Option < Item > { self . remove (key) } fn get_values (& self) -> Vec < (Vec < & Key > , & Value) > { self . get_values () } fn fmt (& mut self) { self . fmt () ; } fn sort_values (& mut self) { self . sort_values () ; } fn is_dotted (& self) -> bool { self . is_dotted () } fn set_dotted (& mut self , yes : bool) { self . set_dotted (yes) ; } fn key (& self , key : & str) -> Option < & '_ Key > { self . key (key) } fn key_mut (& mut self , key : & str) -> Option < KeyMut < '_ > > { self . key_mut (key) } }
    };
}

impl_244!();