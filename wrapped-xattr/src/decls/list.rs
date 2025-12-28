macro_rules! list {
    () => {
        # [doc = " List extended attributes attached to the specified file."] # [doc = ""] # [doc = " Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted"] # [doc = " attributes unless you are root and it may not list system attributes."] pub fn list < P > (path : P) -> io :: Result < XAttrs > where P : AsRef < Path > , { sys :: list_path (path . as_ref () , false) }
    };
}

list!();