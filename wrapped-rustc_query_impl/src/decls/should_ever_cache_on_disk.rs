macro_rules! should_ever_cache_on_disk {
    () => {
        macro_rules ! should_ever_cache_on_disk { ([] $ yes : tt $ no : tt) => { { $ no } } ; ([(cache) $ ($ rest : tt) *] $ yes : tt $ no : tt) => { { $ yes } } ; ([$ other : tt $ ($ modifiers : tt) *] $ yes : tt $ no : tt) => { should_ever_cache_on_disk ! ([$ ($ modifiers) *] $ yes $ no) } ; }
    };
}

should_ever_cache_on_disk!();