macro_rules! AsyncCallableRelevantTypes {
    () => {
        # [doc = " Relevant types for an async callable, including its inputs, output,"] # [doc = " and the return type you get from awaiting the output."] # [derive_where (Clone , Copy , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] pub (in crate :: solve) struct AsyncCallableRelevantTypes < I : Interner > { pub tupled_inputs_ty : I :: Ty , # [doc = " Type returned by calling the closure"] # [doc = " i.e. `f()`."] pub output_coroutine_ty : I :: Ty , # [doc = " Type returned by `await`ing the output"] # [doc = " i.e. `f().await`."] pub coroutine_return_ty : I :: Ty , }
    };
}

AsyncCallableRelevantTypes!();