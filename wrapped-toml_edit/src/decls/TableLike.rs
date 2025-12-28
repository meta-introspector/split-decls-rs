macro_rules! deps {
    () => {
        Value!();
        Iter!();
        IterMut!();
        Entry!();
        Key!();
        Table!();
        InlineTable!();
        KeyMut!();
        Item!();
    };
}

macro_rules! TableLike {
    () => {
        deps!();
        # [doc = " This trait represents either a `Table`, or an `InlineTable`."] pub trait TableLike : crate :: private :: Sealed { # [doc = " Returns an iterator over key/value pairs."] fn iter (& self) -> Iter < '_ > ; # [doc = " Returns an mutable iterator over all key/value pairs, including empty."] fn iter_mut (& mut self) -> IterMut < '_ > ; # [doc = " Returns the number of nonempty items."] fn len (& self) -> usize { self . iter () . filter (| & (_ , v) | ! v . is_none ()) . count () } # [doc = " Returns true if the table is empty."] fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Clears the table, removing all key-value pairs. Keeps the allocated memory for reuse."] fn clear (& mut self) ; # [doc = " Gets the given key's corresponding entry in the Table for in-place manipulation."] fn entry < 'a > (& 'a mut self , key : & str) -> Entry < 'a > ; # [doc = " Gets the given key's corresponding entry in the Table for in-place manipulation."] fn entry_format < 'a > (& 'a mut self , key : & Key) -> Entry < 'a > ; # [doc = " Returns an optional reference to an item given the key."] fn get < 's > (& 's self , key : & str) -> Option < & 's Item > ; # [doc = " Returns an optional mutable reference to an item given the key."] fn get_mut < 's > (& 's mut self , key : & str) -> Option < & 's mut Item > ; # [doc = " Return references to the key-value pair stored for key, if it is present, else None."] fn get_key_value < 'a > (& 'a self , key : & str) -> Option < (& 'a Key , & 'a Item) > ; # [doc = " Return mutable references to the key-value pair stored for key, if it is present, else None."] fn get_key_value_mut < 'a > (& 'a mut self , key : & str) -> Option < (KeyMut < 'a > , & 'a mut Item) > ; # [doc = " Returns true if the table contains an item with the given key."] fn contains_key (& self , key : & str) -> bool ; # [doc = " Inserts a key-value pair into the map."] fn insert (& mut self , key : & str , value : Item) -> Option < Item > ; # [doc = " Removes an item given the key."] fn remove (& mut self , key : & str) -> Option < Item > ; # [doc = " Get key/values for values that are visually children of this table"] # [doc = ""] # [doc = " For example, this will return dotted keys"] fn get_values (& self) -> Vec < (Vec < & Key > , & Value) > ; # [doc = " Auto formats the table."] fn fmt (& mut self) ; # [doc = " Sorts [Key]/[Value]-pairs of the table"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " This sorts the syntactic table (everything under the `[header]`) and not the logical map of"] # [doc = " key-value pairs."] # [doc = " This does not affect the order of [sub-tables][Table] or [sub-arrays][crate::ArrayOfTables]."] # [doc = " This is not recursive."] # [doc = ""] # [doc = " </div>"] fn sort_values (& mut self) ; # [doc = " Change this table's dotted status"] fn set_dotted (& mut self , yes : bool) ; # [doc = " Check if this is a wrapper for dotted keys, rather than a standard table"] fn is_dotted (& self) -> bool ; # [doc = " Returns an accessor to a key's formatting"] fn key (& self , key : & str) -> Option < & '_ Key > ; # [doc = " Returns an accessor to a key's formatting"] fn key_mut (& mut self , key : & str) -> Option < KeyMut < '_ > > ; }
    };
}

TableLike!()