// Generated macro for impl_63 (impl)
macro_rules! Depcrate_arbitraryimpl_63 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_63"}
// Dependencies: {}
impl StrategyItem { # [allow (clippy :: too_many_arguments)] fn new (idx : usize , key : FieldKey , by_ref : bool , is_field : bool , base_idx : Option < usize > , arbitrary_type : Option < Type > , expr : StrategyExpr , dependency : Vec < usize > ,) -> Self { Self { idx , key , by_ref , is_field , base_idx , arbitrary_type , expr , dependency , is_dropped : false , group : None , group_next : None , offset : None , offset_next : None , group_items : Vec :: new () , group_items_next : Vec :: new () , group_dependency : Vec :: new () , group_offset : None , } } fn try_create_independent_strategy (& mut self , ts : & mut TokenStream) -> bool { if self . group . is_none () && self . dependency . is_empty () { let ident = self . strategy_ident () ; let expr = & self . expr ; ts . extend (quote ! (let # ident = # expr ;)) ; self . group = Some (self . idx) ; self . group_next = self . group ; self . offset = Some (0) ; self . offset_next = None ; self . group_items . push (self . idx) ; true } else { false } } fn strategy_ident (& self) -> Ident { parse_str (& format ! ("strategy_{}" , self . idx)) . unwrap () } fn let_sharp_val (& self , from_ref : bool) -> TokenStream { let ident = self . key . to_dummy_ident () ; let expr = if from_ref { quote ! (# ident) } else { quote ! (&# ident) } ; let expr = if self . by_ref { expr } else { quote ! (std :: clone :: Clone :: clone (# expr)) } ; quote ! (let # ident = # expr ;) } }
};
}
