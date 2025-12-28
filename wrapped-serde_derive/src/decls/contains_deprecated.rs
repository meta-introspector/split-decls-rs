macro_rules! contains_deprecated {
    () => {
        # [doc = " Check whether the given attributes contains one of:"] # [doc = "   - `#[deprecated]`"] # [doc = "   - `#[allow(deprecated)]`"] fn contains_deprecated (attrs : & [syn :: Attribute]) -> bool { for attr in attrs { if attr . path () . is_ident ("deprecated") { return true ; } if let syn :: Meta :: List (meta_list) = & attr . meta { if meta_list . path . is_ident ("allow") { let mut allow_deprecated = false ; let _ = meta_list . parse_nested_meta (| meta | { if meta . path . is_ident ("deprecated") { allow_deprecated = true ; } Ok (()) }) ; if allow_deprecated { return true ; } } } } false }
    };
}

contains_deprecated!()