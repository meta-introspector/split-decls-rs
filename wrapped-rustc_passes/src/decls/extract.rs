macro_rules! extract {
    () => {
        # [doc = " Extract the first `rustc_diagnostic_item = \"$name\"` out of a list of attributes."] fn extract (attrs : & [Attribute]) -> Option < Symbol > { attrs . iter () . find_map (| attr | { if attr . has_name (sym :: rustc_diagnostic_item) { attr . value_str () } else { None } }) }
    };
}

extract!()