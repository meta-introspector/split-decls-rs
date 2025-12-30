// Generated macro for impl_21 (impl)
macro_rules! Depcrate_ptrimpl_21 {
() => {
// Module: crate::ptr
// Provides: {"impl_21"}
// Dependencies: {}
impl < N : AstNode > AstPtr < N > { pub fn new (node : & N) -> AstPtr < N > { AstPtr { raw : SyntaxNodePtr :: new (node . syntax ()) , _ty : PhantomData } } pub fn to_node (& self , root : & SyntaxNode) -> N { let syntax_node = self . raw . to_node (root) ; N :: cast (syntax_node) . unwrap () } pub fn syntax_node_ptr (& self) -> SyntaxNodePtr { self . raw } pub fn text_range (& self) -> TextRange { self . raw . text_range () } pub fn cast < U : AstNode > (self) -> Option < AstPtr < U > > { if ! U :: can_cast (self . raw . kind ()) { return None ; } Some (AstPtr { raw : self . raw , _ty : PhantomData }) } pub fn kind (& self) -> parser :: SyntaxKind { self . raw . kind () } pub fn upcast < M : AstNode > (self) -> AstPtr < M > where N : Into < M > , { AstPtr { raw : self . raw , _ty : PhantomData } } # [doc = " Like `SyntaxNodePtr::cast` but the trait bounds work out."] pub fn try_from_raw (raw : SyntaxNodePtr) -> Option < AstPtr < N > > { N :: can_cast (raw . kind ()) . then_some (AstPtr { raw , _ty : PhantomData }) } pub fn wrap_left < R > (self) -> AstPtr < either :: Either < N , R > > where either :: Either < N , R > : AstNode , { AstPtr { raw : self . raw , _ty : PhantomData } } pub fn wrap_right < L > (self) -> AstPtr < either :: Either < L , N > > where either :: Either < L , N > : AstNode , { AstPtr { raw : self . raw , _ty : PhantomData } } }
};
}
