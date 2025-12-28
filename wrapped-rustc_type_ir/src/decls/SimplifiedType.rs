macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! SimplifiedType {
    () => {
        deps!();
        # [doc = " See `simplify_type`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum SimplifiedType < DefId > { Bool , Char , Int (ty :: IntTy) , Uint (ty :: UintTy) , Float (ty :: FloatTy) , Adt (DefId) , Foreign (DefId) , Str , Array , Slice , Ref (Mutability) , Ptr (Mutability) , Never , Tuple (usize) , # [doc = " A trait object, all of whose components are markers"] # [doc = " (e.g., `dyn Send + Sync`)."] MarkerTraitObject , Trait (DefId) , Closure (DefId) , Coroutine (DefId) , CoroutineWitness (DefId) , Function (usize) , UnsafeBinder , Placeholder , Error , }
    };
}

SimplifiedType!()