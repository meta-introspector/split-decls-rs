macro_rules! macro_30 {
    () => {
        decl_derive ! ([Walkable , attributes (visitable)] => # [doc = " Derives `Walkable` for the annotated `struct` or `enum` (`union` is not supported)."] # [doc = ""] # [doc = " Each field of the struct or enum variant will be visited in definition order, using the"] # [doc = " `Walkable` implementation for its type. However, if a field of a struct or an enum"] # [doc = " variant is annotated with `#[visitable(ignore)]` then that field will not be"] # [doc = " visited (and its type is not required to implement `Walkable`)."] visitable :: visitable_derive) ;
    };
}

macro_30!()