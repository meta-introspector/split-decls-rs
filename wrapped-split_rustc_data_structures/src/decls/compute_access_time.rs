macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! compute_access_time {
    () => {
        deps!();
        fn compute_access_time < N : Idx > (start_node : N , immediate_dominators : & IndexSlice < N , Option < N > > ,) -> IndexVec < N , Time > { let mut edges : IndexVec < N , std :: ops :: Range < u32 > > = IndexVec :: from_elem (0 .. 0 , immediate_dominators) ; for & idom in immediate_dominators . iter () { if let Some (idom) = idom { edges [idom] . end += 1 ; } } let mut m = 0 ; for e in edges . iter_mut () { m += e . end ; e . start = m ; e . end = m ; } let mut node = IndexVec :: from_elem_n (Idx :: new (0) , m . try_into () . unwrap ()) ; for (i , & idom) in immediate_dominators . iter_enumerated () { if let Some (idom) = idom { edges [idom] . start -= 1 ; node [edges [idom] . start] = i ; } } let mut time : IndexVec < N , Time > = IndexVec :: from_elem (Time :: default () , immediate_dominators) ; let mut stack = Vec :: new () ; let mut discovered = 1 ; stack . push (start_node) ; time [start_node] . start = discovered ; while let Some (& i) = stack . last () { let e = & mut edges [i] ; if e . start == e . end { time [i] . finish = discovered ; stack . pop () ; } else { let j = node [e . start] ; e . start += 1 ; discovered += 1 ; time [j] . start = discovered ; stack . push (j) ; } } time }
    };
}

compute_access_time!();