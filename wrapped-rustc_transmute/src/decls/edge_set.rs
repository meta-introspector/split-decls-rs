macro_rules! deps {
    () => {
        State!();
        Byte!();
    };
}

macro_rules! edge_set {
    () => {
        deps!();
        mod edge_set { use smallvec :: SmallVec ; use super :: * ; # [doc = " The set of outbound byte edges associated with a DFA node."] # [derive (Eq , PartialEq , Clone , Debug)] pub (super) struct EdgeSet < S = State > { runs : SmallVec < [(Byte , S) ; 1] > , } impl < S > EdgeSet < S > { pub (crate) fn new (range : Byte , dst : S) -> Self { let mut this = Self { runs : SmallVec :: new () } ; if ! range . is_empty () { this . runs . push ((range , dst)) ; } this } pub (crate) fn empty () -> Self { Self { runs : SmallVec :: new () } } # [cfg (test)] pub (crate) fn from_edges (mut edges : Vec < (Byte , S) >) -> Self where S : Ord , { edges . sort () ; Self { runs : edges . into () } } pub (crate) fn iter (& self) -> impl Iterator < Item = (Byte , S) > where S : Copy , { self . runs . iter () . copied () } pub (crate) fn get_uninit_edge_dst (& self) -> Option < S > where S : Copy , { let & (range , dst) = self . runs . last () ? ; if range . contains_uninit () { Some (dst) } else { None } } pub (crate) fn map_states < SS > (self , mut f : impl FnMut (S) -> SS) -> EdgeSet < SS > { EdgeSet { runs : self . runs . into_iter () . map (| (b , s) | (b , f (s))) . collect () , } } # [doc = " Unions two edge sets together."] # [doc = ""] # [doc = " If `u = a.union(b)`, then for each byte value, `u` will have an edge"] # [doc = " with that byte value and with the destination `join(Some(_), None)`,"] # [doc = " `join(None, Some(_))`, or `join(Some(_), Some(_))` depending on whether `a`,"] # [doc = " `b`, or both have an edge with that byte value."] # [doc = ""] # [doc = " If neither `a` nor `b` have an edge with a particular byte value,"] # [doc = " then no edge with that value will be present in `u`."] pub (crate) fn union (& self , other : & Self , mut join : impl FnMut (Option < S > , Option < S >) -> S ,) -> EdgeSet < S > where S : Copy + Eq , { let mut runs : SmallVec < [(Byte , S) ; 1] > = SmallVec :: new () ; let xs = self . runs . iter () . copied () ; let ys = other . runs . iter () . copied () ; for (range , (x , y)) in union (xs , ys) { let state = join (x , y) ; match runs . last_mut () { Some (& mut (ref mut last_range , ref mut last_state)) if last_range . end == range . start && * last_state == state => { last_range . end = range . end } _ => runs . push ((range , state)) , } } EdgeSet { runs } } } }
    };
}

edge_set!();