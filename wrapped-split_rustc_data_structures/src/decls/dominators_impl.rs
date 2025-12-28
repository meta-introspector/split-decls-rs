macro_rules! deps {
    () => {
        Node!();
        Inner!();
        ControlFlowGraph!();
        PreOrderFrame!();
    };
}

macro_rules! dominators_impl {
    () => {
        deps!();
        fn dominators_impl < G : ControlFlowGraph > (graph : & G) -> Inner < G :: Node > { let mut parent : IndexVec < PreorderIndex , PreorderIndex > = IndexVec :: with_capacity (graph . num_nodes ()) ; let mut stack = vec ! [PreOrderFrame { pre_order_idx : PreorderIndex :: ZERO , iter : graph . successors (graph . start_node ()) , }] ; let mut pre_order_to_real : IndexVec < PreorderIndex , G :: Node > = IndexVec :: with_capacity (graph . num_nodes ()) ; let mut real_to_pre_order : IndexVec < G :: Node , Option < PreorderIndex > > = IndexVec :: from_elem_n (None , graph . num_nodes ()) ; pre_order_to_real . push (graph . start_node ()) ; parent . push (PreorderIndex :: ZERO) ; real_to_pre_order [graph . start_node ()] = Some (PreorderIndex :: ZERO) ; 'recurse : while let Some (frame) = stack . last_mut () { for successor in frame . iter . by_ref () { if real_to_pre_order [successor] . is_none () { let pre_order_idx = pre_order_to_real . push (successor) ; real_to_pre_order [successor] = Some (pre_order_idx) ; parent . push (frame . pre_order_idx) ; stack . push (PreOrderFrame { pre_order_idx , iter : graph . successors (successor) }) ; continue 'recurse ; } } stack . pop () ; } let reachable_vertices = pre_order_to_real . len () ; let mut idom = IndexVec :: from_elem_n (PreorderIndex :: ZERO , reachable_vertices) ; let mut semi = IndexVec :: from_fn_n (std :: convert :: identity , reachable_vertices) ; let mut label = semi . clone () ; let mut bucket = IndexVec :: from_elem_n (vec ! [] , reachable_vertices) ; let mut lastlinked = None ; for w in (PreorderIndex :: new (1) .. PreorderIndex :: new (reachable_vertices)) . rev () { for & v in bucket [w] . iter () { let y = eval (& mut parent , lastlinked , & semi , & mut label , v) ; idom [v] = if semi [y] < w { y } else { w } ; } semi [w] = w ; for v in graph . predecessors (pre_order_to_real [w]) { let Some (v) = real_to_pre_order [v] else { continue } ; let x = eval (& mut parent , lastlinked , & semi , & mut label , v) ; semi [w] = std :: cmp :: min (semi [w] , semi [x]) ; } let z = parent [w] ; if z != semi [w] { bucket [semi [w]] . push (w) ; } else { idom [w] = z ; } lastlinked = Some (w) ; } for w in PreorderIndex :: new (1) .. PreorderIndex :: new (reachable_vertices) { if idom [w] != semi [w] { idom [w] = idom [idom [w]] ; } } let mut immediate_dominators = IndexVec :: from_elem_n (None , graph . num_nodes ()) ; for (idx , node) in pre_order_to_real . iter_enumerated () { immediate_dominators [* node] = Some (pre_order_to_real [idom [idx]]) ; } let start_node = graph . start_node () ; immediate_dominators [start_node] = None ; let time = compute_access_time (start_node , & immediate_dominators) ; Inner { immediate_dominators , time } }
    };
}

dominators_impl!();