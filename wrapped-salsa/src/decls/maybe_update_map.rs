macro_rules! maybe_update_map {
    () => {
        macro_rules ! maybe_update_map { ($ old_pointer : expr , $ new_map : expr) => { 'function : { let old_pointer = $ old_pointer ; let new_map = $ new_map ; let old_map : & mut Self = unsafe { & mut * old_pointer } ; let same_keys = old_map . len () == new_map . len () && old_map . keys () . all (| k | new_map . contains_key (k)) ; if ! same_keys { old_map . clear () ; old_map . extend (new_map) ; break 'function true ; } let mut changed = false ; for (key , new_value) in new_map . into_iter () { let old_value = old_map . get_mut (& key) . unwrap () ; changed |= unsafe { V :: maybe_update (old_value , new_value) } ; } changed } } ; }
    };
}

maybe_update_map!()