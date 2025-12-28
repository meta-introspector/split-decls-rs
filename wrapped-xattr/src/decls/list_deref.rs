macro_rules! list_deref {
    () => {
        # [doc = " List extended attributes attached to the specified file (dereference symlinks)."] pub fn list_deref < P > (path : P) -> io :: Result < XAttrs > where P : AsRef < Path > , { sys :: list_path (path . as_ref () , true) }
    };
}

list_deref!()