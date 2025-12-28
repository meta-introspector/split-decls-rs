macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! StatCollector {
    () => {
        deps!();
        # [doc = " This type measures the size of AST and HIR nodes, by implementing the AST"] # [doc = " and HIR `Visitor` traits. But we don't measure every visited type because"] # [doc = " that could cause double counting."] # [doc = ""] # [doc = " For example, `ast::Visitor` has `visit_ident`, but `Ident`s are always"] # [doc = " stored inline within other AST nodes, so we don't implement `visit_ident`"] # [doc = " here. In contrast, we do implement `visit_expr` because `ast::Expr` is"] # [doc = " always stored as `Box<ast::Expr>`, and every such expression should be"] # [doc = " measured separately."] # [doc = ""] # [doc = " In general, a `visit_foo` method should be implemented here if the"] # [doc = " corresponding `Foo` type is always stored on its own, e.g.: `Box<Foo>`,"] # [doc = " `Box<Foo>`, `Vec<Foo>`, `Box<[Foo]>`."] # [doc = ""] # [doc = " There are some types in the AST and HIR tree that the visitors do not have"] # [doc = " a `visit_*` method for, and so we cannot measure these, which is"] # [doc = " unfortunate."] struct StatCollector < 'k > { tcx : Option < TyCtxt < 'k > > , nodes : FxHashMap < & 'static str , Node > , seen : FxHashSet < HirId > , }
    };
}

StatCollector!()