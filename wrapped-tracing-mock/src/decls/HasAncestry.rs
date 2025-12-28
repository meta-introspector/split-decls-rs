macro_rules! HasAncestry {
    () => {
        pub (crate) trait HasAncestry { fn is_contextual (& self) -> bool ; fn is_root (& self) -> bool ; fn parent (& self) -> Option < & span :: Id > ; }
    };
}

HasAncestry!();