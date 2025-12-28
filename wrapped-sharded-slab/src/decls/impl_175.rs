macro_rules! deps {
    () => {
        Registration!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        # [cfg (not (all (loom , any (feature = "loom" , test))))] impl Drop for Registration { fn drop (& mut self) { use std :: sync :: PoisonError ; if let Some (id) = self . 0 . get () { let mut free_list = REGISTRY . free . lock () . unwrap_or_else (PoisonError :: into_inner) ; free_list . push_back (id) ; } } }
    };
}

impl_175!()