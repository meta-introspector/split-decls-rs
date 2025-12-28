macro_rules! deps {
    () => {
        Stack!();
        Cx!();
        AvailableDepth!();
        Delegate!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl AvailableDepth { # [doc = " Returns the remaining depth allowed for nested goals."] # [doc = ""] # [doc = " This is generally simply one less than the current depth."] # [doc = " However, if we encountered overflow, we significantly reduce"] # [doc = " the remaining depth of all nested goals to prevent hangs"] # [doc = " in case there is exponential blowup."] fn allowed_depth_for_nested < D : Delegate > (root_depth : AvailableDepth , stack : & Stack < D :: Cx > ,) -> Option < AvailableDepth > { if let Some (last) = stack . last () { if last . available_depth . 0 == 0 { return None ; } Some (if last . encountered_overflow { AvailableDepth (last . available_depth . 0 / D :: DIVIDE_AVAILABLE_DEPTH_ON_OVERFLOW) } else { AvailableDepth (last . available_depth . 0 - 1) }) } else { Some (root_depth) } } # [doc = " Whether we're allowed to use a global cache entry which required"] # [doc = " the given depth."] fn cache_entry_is_applicable (self , additional_depth : usize) -> bool { self . 0 >= additional_depth } }
    };
}

impl_150!()