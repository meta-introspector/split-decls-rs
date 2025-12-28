macro_rules! ItemLike {
    () => {
        # [derive (Clone , Copy)] enum ItemLike < 'tcx > { Item (& 'tcx Item < 'tcx >) , ForeignItem , }
    };
}

ItemLike!()