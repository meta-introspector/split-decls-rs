macro_rules! filter_array_elements {
    () => {
        # [doc = " HACK: in order to not mistakenly assume that `[PhantomData<T>; N]` requires drop glue"] # [doc = " we check the element type for drop glue. The correct fix would be looking at the"] # [doc = " entirety of the code around `needs_drop_components` and this file and come up with"] # [doc = " logic that is easier to follow while not repeating any checks that may thus diverge."] fn filter_array_elements < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> impl Fn (& Result < Ty < 'tcx > , AlwaysRequiresDrop >) -> bool { move | ty | match ty { Ok (ty) => match * ty . kind () { ty :: Array (elem , _) => tcx . needs_drop_raw (typing_env . as_query_input (elem)) , _ => true , } , Err (AlwaysRequiresDrop) => true , } }
    };
}

filter_array_elements!();