macro_rules! type_matches_path {
    () => {
        # [doc = " Checks whether the type name of `ty` matches `name`."] # [doc = ""] # [doc = " Given some struct at `a::b::c::Foo`, this will return true for `c::Foo`, `b::c::Foo`, or"] # [doc = " `a::b::c::Foo`. This reasonably allows qualified names to be used in the macro."] pub (crate) fn type_matches_path (ty : & Type , name : & [& str]) -> bool { if let Type :: Path (ty) = ty { ty . path . segments . iter () . map (| s | s . ident . to_string ()) . rev () . zip (name . iter () . rev ()) . all (| (x , y) | & x . as_str () == y) } else { false } }
    };
}

type_matches_path!()