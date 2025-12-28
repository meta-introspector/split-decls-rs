macro_rules! deps {
    () => {
        Slab!();
        Builder!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [doc = " Create a slab from an iterator of key-value pairs."] # [doc = ""] # [doc = " If the iterator produces duplicate keys, the previous value is replaced with the later one."] # [doc = " The keys does not need to be sorted beforehand, and this function always"] # [doc = " takes O(n) time."] # [doc = " Note that the returned slab will use space proportional to the largest key,"] # [doc = " so don't use `Slab` with untrusted keys."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use slab::*;"] # [doc = ""] # [doc = " let vec = vec![(2,'a'), (6,'b'), (7,'c')];"] # [doc = " let slab = vec.into_iter().collect::<Slab<char>>();"] # [doc = " assert_eq!(slab.len(), 3);"] # [doc = " assert!(slab.capacity() >= 8);"] # [doc = " assert_eq!(slab[2], 'a');"] # [doc = " ```"] # [doc = ""] # [doc = " With duplicate and unsorted keys:"] # [doc = ""] # [doc = " ```"] # [doc = " # use slab::*;"] # [doc = ""] # [doc = " let vec = vec![(20,'a'), (10,'b'), (11,'c'), (10,'d')];"] # [doc = " let slab = vec.into_iter().collect::<Slab<char>>();"] # [doc = " assert_eq!(slab.len(), 3);"] # [doc = " assert_eq!(slab[10], 'd');"] # [doc = " ```"] impl < T > FromIterator < (usize , T) > for Slab < T > { fn from_iter < I > (iterable : I) -> Self where I : IntoIterator < Item = (usize , T) > , { let iterator = iterable . into_iter () ; let mut builder = builder :: Builder :: with_capacity (iterator . size_hint () . 0) ; for (key , value) in iterator { builder . pair (key , value) ; } builder . build () } }
    };
}

impl_30!()