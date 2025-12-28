macro_rules! SortedMap {
    () => {
        # [doc = " `SortedMap` is a data structure with similar characteristics as BTreeMap but"] # [doc = " slightly different trade-offs: lookup is *O*(log(*n*)), insertion and removal"] # [doc = " are *O*(*n*) but elements can be iterated in order cheaply."] # [doc = ""] # [doc = " `SortedMap` can be faster than a `BTreeMap` for small sizes (<50) since it"] # [doc = " stores data in a more compact way. It also supports accessing contiguous"] # [doc = " ranges of elements as a slice, and slices of already sorted elements can be"] # [doc = " inserted efficiently."] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable_NoContext , Decodable_NoContext)] pub struct SortedMap < K , V > { data : Vec < (K , V) > , }
    };
}

SortedMap!();