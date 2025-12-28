macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! table_entry {
    () => {
        deps!();
        # [inline] fn table_entry < 'a , K , V , Q > (table : & 'a mut HashTable < (K , V) > , hash : u64 , key : & Q ,) -> Entry < 'a , (K , V) > where K : Hash + Borrow < Q > , Q : ? Sized + Eq , { table . entry (hash , move | (k , _) | k . borrow () == key , | (k , _) | make_hash (k)) }
    };
}

table_entry!()