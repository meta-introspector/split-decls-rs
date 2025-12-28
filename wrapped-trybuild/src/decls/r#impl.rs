macro_rules! deps {
    () => {
        Render!();
    };
}

macro_rules! r#impl {
    () => {
        deps!();
        # [cfg (any (not (feature = "diff") , windows))] mod r#impl { use super :: Render ; pub (crate) enum Diff { } impl Diff { pub fn compute (_expected : & str , _actual : & str) -> Option < Self > { None } pub fn iter (& self , _input : & str) -> Box < dyn Iterator < Item = Render > > { let _ = Render :: Common ; let _ = Render :: Unique ; match * self { } } } }
    };
}

r#impl!();