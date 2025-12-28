macro_rules! deps {
    () => {
        TrackElem!();
        PlaceInfo!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " Partial mapping from [`Place`] to [`PlaceIndex`], where some places also have a [`ValueIndex`]."] # [doc = ""] # [doc = " This data structure essentially maintains a tree of places and their projections. Some"] # [doc = " additional bookkeeping is done, to speed up traversal over this tree:"] # [doc = " - For iteration, every [`PlaceInfo`] contains an intrusive linked list of its children."] # [doc = " - To directly get the child for a specific projection, there is a `projections` map."] # [derive (Debug)] pub struct Map < 'tcx > { locals : IndexVec < Local , Option < PlaceIndex > > , projections : FxHashMap < (PlaceIndex , TrackElem) , PlaceIndex > , places : IndexVec < PlaceIndex , PlaceInfo < 'tcx > > , value_count : usize , inner_values : IndexVec < PlaceIndex , Range < usize > > , inner_values_buffer : Vec < ValueIndex > , }
    };
}

Map!();