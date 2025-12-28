macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! macro_743 {
    () => {
        deps!();
        ast_enum_of_structs ! { # [doc = " The possible types that a Rust value could have."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] # [non_exhaustive] pub enum Type { # [doc = " A fixed size array type: `[T; n]`."] Array (TypeArray) , # [doc = " A bare function type: `fn(usize) -> bool`."] BareFn (TypeBareFn) , # [doc = " A type contained within invisible delimiters."] Group (TypeGroup) , # [doc = " An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or"] # [doc = " a lifetime."] ImplTrait (TypeImplTrait) , # [doc = " Indication that a type should be inferred by the compiler: `_`."] Infer (TypeInfer) , # [doc = " A macro in the type position."] Macro (TypeMacro) , # [doc = " The never type: `!`."] Never (TypeNever) , # [doc = " A parenthesized type equivalent to the inner type."] Paren (TypeParen) , # [doc = " A path like `std::slice::Iter`, optionally qualified with a"] # [doc = " self-type as in `<Vec<T> as SomeTrait>::Associated`."] Path (TypePath) , # [doc = " A raw pointer type: `*const T` or `*mut T`."] Ptr (TypePtr) , # [doc = " A reference type: `&'a T` or `&'a mut T`."] Reference (TypeReference) , # [doc = " A dynamically sized slice type: `[T]`."] Slice (TypeSlice) , # [doc = " A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a"] # [doc = " trait or a lifetime."] TraitObject (TypeTraitObject) , # [doc = " A tuple type: `(A, B, C, String)`."] Tuple (TypeTuple) , # [doc = " Tokens in type position not interpreted by Syn."] Verbatim (TokenStream) , } }
    };
}

macro_743!()