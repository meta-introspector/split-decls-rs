macro_rules! macro_586 {
    () => {
        ast_struct ! { # [doc = " The explicit Self type in a qualified path: the `T` in `<T as"] # [doc = " Display>::fmt`."] # [doc = ""] # [doc = " The actual path, including the trait and the associated item, is stored"] # [doc = " separately. The `position` field represents the index of the associated"] # [doc = " item qualified with this Self type."] # [doc = ""] # [doc = " ```text"] # [doc = " <Vec<T> as a::b::Trait>::AssociatedItem"] # [doc = "  ^~~~~~    ~~~~~~~~~~~~~~^"] # [doc = "  ty        position = 3"] # [doc = ""] # [doc = " <Vec<T>>::AssociatedItem"] # [doc = "  ^~~~~~   ^"] # [doc = "  ty       position = 0"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct QSelf { pub lt_token : Token ! [<] , pub ty : Box < Type >, pub position : usize , pub as_token : Option < Token ! [as] >, pub gt_token : Token ! [>] , } }
    };
}

macro_586!()