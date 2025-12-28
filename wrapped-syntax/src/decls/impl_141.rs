macro_rules! deps {
    () => {
        SyntaxEdit!();
        SyntaxElement!();
        Change!();
        SyntaxEditor!();
        SyntaxAnnotation!();
        Position!();
        Element!();
        SyntaxNode!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl SyntaxEditor { # [doc = " Creates a syntax editor to start editing from `root`"] pub fn new (root : SyntaxNode) -> Self { Self { root , changes : vec ! [] , mappings : SyntaxMapping :: default () , annotations : vec ! [] } } pub fn add_annotation (& mut self , element : impl Element , annotation : SyntaxAnnotation) { self . annotations . push ((element . syntax_element () , annotation)) } pub fn add_annotation_all (& mut self , elements : Vec < impl Element > , annotation : SyntaxAnnotation ,) { self . annotations . extend (elements . into_iter () . map (| e | e . syntax_element ()) . zip (iter :: repeat (annotation))) ; } pub fn merge (& mut self , mut other : SyntaxEditor) { debug_assert ! (self . root == other . root || other . root . ancestors () . any (| node | node == self . root) , "{:?} is not in the same tree as {:?}" , other . root , self . root) ; self . changes . append (& mut other . changes) ; self . mappings . merge (other . mappings) ; self . annotations . append (& mut other . annotations) ; } pub fn insert (& mut self , position : Position , element : impl Element) { debug_assert ! (is_ancestor_or_self (& position . parent () , & self . root)) ; self . changes . push (Change :: Insert (position , element . syntax_element ())) } pub fn insert_all (& mut self , position : Position , elements : Vec < SyntaxElement >) { debug_assert ! (is_ancestor_or_self (& position . parent () , & self . root)) ; self . changes . push (Change :: InsertAll (position , elements)) } pub fn delete (& mut self , element : impl Element) { let element = element . syntax_element () ; debug_assert ! (is_ancestor_or_self_of_element (& element , & self . root)) ; debug_assert ! (! matches ! (& element , SyntaxElement :: Node (node) if node == & self . root) , "should not delete root node") ; self . changes . push (Change :: Replace (element . syntax_element () , None)) ; } pub fn delete_all (& mut self , range : RangeInclusive < SyntaxElement >) { if range . start () == range . end () { self . delete (range . start ()) ; return ; } debug_assert ! (is_ancestor_or_self_of_element (range . start () , & self . root)) ; self . changes . push (Change :: ReplaceAll (range , Vec :: new ())) } pub fn replace (& mut self , old : impl Element , new : impl Element) { let old = old . syntax_element () ; debug_assert ! (is_ancestor_or_self_of_element (& old , & self . root)) ; self . changes . push (Change :: Replace (old . syntax_element () , Some (new . syntax_element ()))) ; } pub fn replace_with_many (& mut self , old : impl Element , new : Vec < SyntaxElement >) { let old = old . syntax_element () ; debug_assert ! (is_ancestor_or_self_of_element (& old , & self . root)) ; debug_assert ! (! (matches ! (& old , SyntaxElement :: Node (node) if node == & self . root) && new . len () > 1) , "cannot replace root node with many elements") ; self . changes . push (Change :: ReplaceWithMany (old . syntax_element () , new)) ; } pub fn replace_all (& mut self , range : RangeInclusive < SyntaxElement > , new : Vec < SyntaxElement >) { if range . start () == range . end () { self . replace_with_many (range . start () , new) ; return ; } debug_assert ! (is_ancestor_or_self_of_element (range . start () , & self . root)) ; self . changes . push (Change :: ReplaceAll (range , new)) } pub fn finish (self) -> SyntaxEdit { edit_algo :: apply_edits (self) } pub fn add_mappings (& mut self , other : SyntaxMapping) { self . mappings . merge (other) ; } }
    };
}

impl_141!();