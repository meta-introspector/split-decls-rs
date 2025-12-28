macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! MirUsedCollector {
    () => {
        deps!();
        struct MirUsedCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a mir :: Body < 'tcx > , used_items : & 'a mut MonoItems < 'tcx > , # [doc = " See the comment in `collect_items_of_instance` for the purpose of this set."] # [doc = " Note that this contains *not-monomorphized* items!"] used_mentioned_items : & 'a mut UnordSet < MentionedItem < 'tcx > > , instance : Instance < 'tcx > , }
    };
}

MirUsedCollector!()