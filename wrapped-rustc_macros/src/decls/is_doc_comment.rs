macro_rules! is_doc_comment {
    () => {
        pub (super) fn is_doc_comment (attr : & Attribute) -> bool { attr . path () . segments . last () . unwrap () . ident == "doc" }
    };
}

is_doc_comment!();