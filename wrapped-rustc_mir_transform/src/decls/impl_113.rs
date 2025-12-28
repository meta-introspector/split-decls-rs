macro_rules! deps {
    () => {
        MirPatch!();
        DropStyle!();
        DropFlagMode!();
        DropElaborator!();
        DropShimElaborator!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < 'a , 'tcx > DropElaborator < 'a , 'tcx > for DropShimElaborator < 'a , 'tcx > { type Path = () ; fn patch_ref (& self) -> & MirPatch < 'tcx > { & self . patch } fn patch (& mut self) -> & mut MirPatch < 'tcx > { & mut self . patch } fn body (& self) -> & 'a Body < 'tcx > { self . body } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env } fn terminator_loc (& self , bb : BasicBlock) -> Location { self . patch . terminator_loc (self . body , bb) } fn allow_async_drops (& self) -> bool { self . produce_async_drops } fn drop_style (& self , _path : Self :: Path , mode : DropFlagMode) -> DropStyle { match mode { DropFlagMode :: Shallow => { DropStyle :: Static } DropFlagMode :: Deep => { DropStyle :: Open } } } fn get_drop_flag (& mut self , _path : Self :: Path) -> Option < Operand < 'tcx > > { None } fn clear_drop_flag (& mut self , _location : Location , _path : Self :: Path , _mode : DropFlagMode) { } fn field_subpath (& self , _path : Self :: Path , _field : FieldIdx) -> Option < Self :: Path > { None } fn deref_subpath (& self , _path : Self :: Path) -> Option < Self :: Path > { None } fn downcast_subpath (& self , _path : Self :: Path , _variant : VariantIdx) -> Option < Self :: Path > { Some (()) } fn array_subpath (& self , _path : Self :: Path , _index : u64 , _size : u64) -> Option < Self :: Path > { None } }
    };
}

impl_113!();