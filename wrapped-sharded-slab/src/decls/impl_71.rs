macro_rules! deps {
    () => {
        Config!();
        UniqueIter!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a , T , C : cfg :: Config > Iterator for UniqueIter < 'a , T , C > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { test_println ! ("UniqueIter::next") ; loop { test_println ! ("-> try next slot") ; if let Some (item) = self . slots . as_mut () . and_then (| slots | slots . next ()) { test_println ! ("-> found an item!") ; return Some (item) ; } test_println ! ("-> try next page") ; if let Some (page) = self . pages . next () { test_println ! ("-> found another page") ; self . slots = page . iter () ; continue ; } test_println ! ("-> try next shard") ; if let Some (shard) = self . shards . next () { test_println ! ("-> found another shard") ; self . pages = shard . iter () ; } else { test_println ! ("-> all done!") ; return None ; } } } }
    };
}

impl_71!()