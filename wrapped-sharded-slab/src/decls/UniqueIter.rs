macro_rules! deps {
    () => {
        Shared!();
        Iter!();
        IterMut!();
        Config!();
    };
}

macro_rules! UniqueIter {
    () => {
        deps!();
        # [doc = " An exclusive fused iterator over the items in a [`Slab`](crate::Slab)."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [derive (Debug)] pub struct UniqueIter < 'a , T , C : cfg :: Config > { pub (super) shards : shard :: IterMut < 'a , Option < T > , C > , pub (super) pages : slice :: Iter < 'a , page :: Shared < Option < T > , C > > , pub (super) slots : Option < page :: Iter < 'a , T , C > > , }
    };
}

UniqueIter!();