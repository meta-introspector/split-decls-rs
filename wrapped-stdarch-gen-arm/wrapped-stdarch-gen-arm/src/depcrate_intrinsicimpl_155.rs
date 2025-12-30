// Generated macro for impl_155 (impl)
macro_rules! Depcrate_intrinsicimpl_155 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_155"}
// Dependencies: {}
impl Safety { # [doc = " Return `Ok(Safety::Safe)` if safety appears reasonable for the given `intrinsic`'s name and"] # [doc = " prototype. Otherwise, return `Err()` with a suitable diagnostic."] fn safe_checked (intrinsic : & Intrinsic) -> Result < Self , String > { let name = intrinsic . signature . doc_name () ; if name . starts_with ("sv") { let handles_pointers = intrinsic . signature . arguments . iter () . any (| arg | matches ! (arg . kind , TypeKind :: Pointer (..))) ; if name . starts_with ("svld") || name . starts_with ("svst") || name . starts_with ("svprf") || name . starts_with ("svundef") || handles_pointers { let doc = intrinsic . doc . as_ref () . map (| s | s . to_string ()) ; let doc = doc . as_deref () . unwrap_or ("...") ; Err (format ! ("`{name}` has no safety specification, but it looks like it should be unsafe. \
                Consider specifying (un)safety explicitly:

  - name: {name}
    doc: {doc}
    safety:
      unsafe:
        - ...
    ...
")) } else { Ok (Self :: Safe) } } else { Err (format ! ("Safety::safe_checked() for non-SVE intrinsic: {name}")) } } fn is_safe (& self) -> bool { match self { Self :: Safe => true , Self :: Unsafe (..) => false , } } fn is_unsafe (& self) -> bool { ! self . is_safe () } fn has_doc_comments (& self) -> bool { match self { Self :: Safe => false , Self :: Unsafe (v) => ! v . is_empty () , } } fn doc_comments (& self) -> & [UnsafetyComment] { match self { Self :: Safe => & [] , Self :: Unsafe (v) => v . as_slice () , } } }
};
}
