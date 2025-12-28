macro_rules! deps {
    () => {
        UnionFindEntry!();
    };
}

macro_rules! UnionFind {
    () => {
        deps!();
        # [doc = " Simple implementation of a union-find data structure, i.e. a disjoint-set"] # [doc = " forest."] # [derive (Debug)] pub struct UnionFind < Key : Idx > { table : IndexVec < Key , UnionFindEntry < Key > > , }
    };
}

UnionFind!()