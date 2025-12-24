use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> VariantInfo<'a> {
    fn new(
        ast: VariantAst<'a>,
        prefix: Option<&'a Ident>,
        generics: &'a Generics,
    ) -> Self {
        let bindings = match ast.fields {
            Fields::Unit => vec![],
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. })
            | Fields::Named(FieldsNamed { named: fields, .. }) => {
                fields
                    .into_iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let binding_span = Span::call_site().located_at(field.span());
                        BindingInfo {
                            binding: format_ident!(
                                "__binding_{}", i, span = binding_span
                            ),
                            style: BindStyle::Ref,
                            field,
                            generics,
                            seen_generics: get_ty_params(field, generics),
                            index: i,
                        }
                    })
                    .collect::<Vec<_>>()
            }
        };
        let original_length = bindings.len();
        VariantInfo {
            prefix,
            bindings,
            ast,
            generics,
            original_length,
        }
    }
    /// Returns a slice of the bindings in this Variant.
    pub fn bindings(&self) -> &[BindingInfo<'a>] {
        &self.bindings
    }
    /// Returns a mut slice of the bindings in this Variant.
    pub fn bindings_mut(&mut self) -> &mut [BindingInfo<'a>] {
        &mut self.bindings
    }
    /// Returns a `VariantAst` object which contains references to the
    /// underlying `syn` AST node which this `Variant` was created from.
    pub fn ast(&self) -> VariantAst<'a> {
        self.ast
    }
    /// True if any bindings were omitted due to a `filter` call.
    pub fn omitted_bindings(&self) -> bool {
        self.original_length != self.bindings.len()
    }
    /// Generates the match-arm pattern which could be used to match against this Variant.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B(i32, i32),
    ///         C(u32),
    ///     }
    /// };
    /// let s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].pat().to_string(),
    ///     quote!{
    ///         A::B(ref __binding_0, ref __binding_1,)
    ///     }.to_string()
    /// );
    /// ```
    pub fn pat(&self) -> TokenStream {
        let mut t = TokenStream::new();
        if let Some(prefix) = self.prefix {
            prefix.to_tokens(&mut t);
            quote!(::).to_tokens(&mut t);
        }
        self.ast.ident.to_tokens(&mut t);
        match self.ast.fields {
            Fields::Unit => {
                assert!(self.bindings.is_empty());
            }
            Fields::Unnamed(..) => {
                token::Paren(Span::call_site())
                    .surround(
                        &mut t,
                        |t| {
                            let mut expected_index = 0;
                            for binding in &self.bindings {
                                while expected_index < binding.index {
                                    quote!(_,).to_tokens(t);
                                    expected_index += 1;
                                }
                                binding.pat().to_tokens(t);
                                quote!(,).to_tokens(t);
                                expected_index += 1;
                            }
                            if expected_index != self.original_length {
                                quote!(..).to_tokens(t);
                            }
                        },
                    )
            }
            Fields::Named(..) => {
                token::Brace(Span::call_site())
                    .surround(
                        &mut t,
                        |t| {
                            for binding in &self.bindings {
                                binding.field.ident.to_tokens(t);
                                quote!(:).to_tokens(t);
                                binding.pat().to_tokens(t);
                                quote!(,).to_tokens(t);
                            }
                            if self.omitted_bindings() {
                                quote!(..).to_tokens(t);
                            }
                        },
                    )
            }
        }
        t
    }
    /// Generates the token stream required to construct the current variant.
    ///
    /// The init array initializes each of the fields in the order they are
    /// written in `variant.ast().fields`.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B(usize, usize),
    ///         C{ v: usize },
    ///     }
    /// };
    /// let s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].construct(|_, i| quote!(#i)).to_string(),
    ///
    ///     quote!{
    ///         A::B(0usize, 1usize,)
    ///     }.to_string()
    /// );
    ///
    /// assert_eq!(
    ///     s.variants()[1].construct(|_, i| quote!(#i)).to_string(),
    ///
    ///     quote!{
    ///         A::C{ v: 0usize, }
    ///     }.to_string()
    /// );
    /// ```
    pub fn construct<F, T>(&self, mut func: F) -> TokenStream
    where
        F: FnMut(&Field, usize) -> T,
        T: ToTokens,
    {
        let mut t = TokenStream::new();
        if let Some(prefix) = self.prefix {
            quote!(# prefix::).to_tokens(&mut t);
        }
        self.ast.ident.to_tokens(&mut t);
        match &self.ast.fields {
            Fields::Unit => {}
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                token::Paren::default()
                    .surround(
                        &mut t,
                        |t| {
                            for (i, field) in unnamed.into_iter().enumerate() {
                                func(field, i).to_tokens(t);
                                quote!(,).to_tokens(t);
                            }
                        },
                    );
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                token::Brace::default()
                    .surround(
                        &mut t,
                        |t| {
                            for (i, field) in named.into_iter().enumerate() {
                                field.ident.to_tokens(t);
                                quote!(:).to_tokens(t);
                                func(field, i).to_tokens(t);
                                quote!(,).to_tokens(t);
                            }
                        },
                    );
            }
        }
        t
    }
    /// Runs the passed-in function once for each bound field, passing in a `BindingInfo`.
    /// and generating a `match` arm which evaluates the returned tokens.
    ///
    /// This method will ignore fields which are ignored through the `filter`
    /// method.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B(i32, i32),
    ///         C(u32),
    ///     }
    /// };
    /// let s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].each(|bi| quote!(println!("{:?}", #bi))).to_string(),
    ///
    ///     quote!{
    ///         A::B(ref __binding_0, ref __binding_1,) => {
    ///             { println!("{:?}", __binding_0) }
    ///             { println!("{:?}", __binding_1) }
    ///         }
    ///     }.to_string()
    /// );
    /// ```
    pub fn each<F, R>(&self, mut f: F) -> TokenStream
    where
        F: FnMut(&BindingInfo<'_>) -> R,
        R: ToTokens,
    {
        let pat = self.pat();
        let mut body = TokenStream::new();
        for binding in &self.bindings {
            token::Brace::default()
                .surround(
                    &mut body,
                    |body| {
                        f(binding).to_tokens(body);
                    },
                );
        }
        quote!(# pat => { # body })
    }
    /// Runs the passed-in function once for each bound field, passing in the
    /// result of the previous call, and a `BindingInfo`. generating a `match`
    /// arm which evaluates to the resulting tokens.
    ///
    /// This method will ignore fields which are ignored through the `filter`
    /// method.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B(i32, i32),
    ///         C(u32),
    ///     }
    /// };
    /// let s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].fold(quote!(0), |acc, bi| quote!(#acc + #bi)).to_string(),
    ///
    ///     quote!{
    ///         A::B(ref __binding_0, ref __binding_1,) => {
    ///             0 + __binding_0 + __binding_1
    ///         }
    ///     }.to_string()
    /// );
    /// ```
    pub fn fold<F, I, R>(&self, init: I, mut f: F) -> TokenStream
    where
        F: FnMut(TokenStream, &BindingInfo<'_>) -> R,
        I: ToTokens,
        R: ToTokens,
    {
        let pat = self.pat();
        let body = self
            .bindings
            .iter()
            .fold(
                quote!(# init),
                |i, bi| {
                    let r = f(i, bi);
                    quote!(# r)
                },
            );
        quote!(# pat => { # body })
    }
    /// Filter the bindings created by this `Variant` object. This has 2 effects:
    ///
    /// * The bindings will no longer appear in match arms generated by methods
    ///   on this `Variant` or its subobjects.
    ///
    /// * Impl blocks created with the `bound_impl` or `unsafe_bound_impl`
    ///   method only consider type parameters referenced in the types of
    ///   non-filtered fields.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B{ a: i32, b: i32 },
    ///         C{ a: u32 },
    ///     }
    /// };
    /// let mut s = Structure::new(&di);
    ///
    /// s.variants_mut()[0].filter(|bi| {
    ///     bi.ast().ident == Some(quote::format_ident!("b"))
    /// });
    ///
    /// assert_eq!(
    ///     s.each(|bi| quote!(println!("{:?}", #bi))).to_string(),
    ///
    ///     quote!{
    ///         A::B{ b: ref __binding_1, .. } => {
    ///             { println!("{:?}", __binding_1) }
    ///         }
    ///         A::C{ a: ref __binding_0, } => {
    ///             { println!("{:?}", __binding_0) }
    ///         }
    ///     }.to_string()
    /// );
    /// ```
    pub fn filter<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&BindingInfo<'_>) -> bool,
    {
        self.bindings.retain(f);
        self
    }
    /// Iterates all the bindings of this `Variant` object and uses a closure to determine if a
    /// binding should be removed. If the closure returns `true` the binding is removed from the
    /// variant. If the closure returns `false`, the binding remains in the variant.
    ///
    /// All the removed bindings are moved to a new `Variant` object which is otherwise identical
    /// to the current one. To understand the effects of removing a binding from a variant check
    /// the [`VariantInfo::filter`] documentation.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B{ a: i32, b: i32 },
    ///         C{ a: u32 },
    ///     }
    /// };
    /// let mut s = Structure::new(&di);
    ///
    /// let mut with_b = &mut s.variants_mut()[0];
    ///
    /// let with_a = with_b.drain_filter(|bi| {
    ///     bi.ast().ident == Some(quote::format_ident!("a"))
    /// });
    ///
    /// assert_eq!(
    ///     with_a.each(|bi| quote!(println!("{:?}", #bi))).to_string(),
    ///
    ///     quote!{
    ///         A::B{ a: ref __binding_0, .. } => {
    ///             { println!("{:?}", __binding_0) }
    ///         }
    ///     }.to_string()
    /// );
    ///
    /// assert_eq!(
    ///     with_b.each(|bi| quote!(println!("{:?}", #bi))).to_string(),
    ///
    ///     quote!{
    ///         A::B{ b: ref __binding_1, .. } => {
    ///             { println!("{:?}", __binding_1) }
    ///         }
    ///     }.to_string()
    /// );
    /// ```
    #[allow(clippy::return_self_not_must_use)]
    pub fn drain_filter<F>(&mut self, mut f: F) -> Self
    where
        F: FnMut(&BindingInfo<'_>) -> bool,
    {
        let mut other = VariantInfo {
            prefix: self.prefix,
            bindings: vec![],
            ast: self.ast,
            generics: self.generics,
            original_length: self.original_length,
        };
        let (other_bindings, self_bindings) = self.bindings.drain(..).partition(&mut f);
        other.bindings = other_bindings;
        self.bindings = self_bindings;
        other
    }
    /// Remove the binding at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of range.
    pub fn remove_binding(&mut self, idx: usize) -> &mut Self {
        self.bindings.remove(idx);
        self
    }
    /// Updates the `BindStyle` for each of the passed-in fields by calling the
    /// passed-in function for each `BindingInfo`.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B(i32, i32),
    ///         C(u32),
    ///     }
    /// };
    /// let mut s = Structure::new(&di);
    ///
    /// s.variants_mut()[0].bind_with(|bi| BindStyle::RefMut);
    ///
    /// assert_eq!(
    ///     s.each(|bi| quote!(println!("{:?}", #bi))).to_string(),
    ///
    ///     quote!{
    ///         A::B(ref mut __binding_0, ref mut __binding_1,) => {
    ///             { println!("{:?}", __binding_0) }
    ///             { println!("{:?}", __binding_1) }
    ///         }
    ///         A::C(ref __binding_0,) => {
    ///             { println!("{:?}", __binding_0) }
    ///         }
    ///     }.to_string()
    /// );
    /// ```
    pub fn bind_with<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&BindingInfo<'_>) -> BindStyle,
    {
        for binding in &mut self.bindings {
            binding.style = f(binding);
        }
        self
    }
    /// Updates the binding name for each fo the passed-in fields by calling the
    /// passed-in function for each `BindingInfo`.
    ///
    /// The function will be called with the `BindingInfo` and its index in the
    /// enclosing variant.
    ///
    /// The default name is `__binding_{}` where `{}` is replaced with an
    /// increasing number.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B{ a: i32, b: i32 },
    ///         C{ a: u32 },
    ///     }
    /// };
    /// let mut s = Structure::new(&di);
    ///
    /// s.variants_mut()[0].binding_name(|bi, i| bi.ident.clone().unwrap());
    ///
    /// assert_eq!(
    ///     s.each(|bi| quote!(println!("{:?}", #bi))).to_string(),
    ///
    ///     quote!{
    ///         A::B{ a: ref a, b: ref b, } => {
    ///             { println!("{:?}", a) }
    ///             { println!("{:?}", b) }
    ///         }
    ///         A::C{ a: ref __binding_0, } => {
    ///             { println!("{:?}", __binding_0) }
    ///         }
    ///     }.to_string()
    /// );
    /// ```
    pub fn binding_name<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&Field, usize) -> Ident,
    {
        for (it, binding) in self.bindings.iter_mut().enumerate() {
            binding.binding = f(binding.field, it);
        }
        self
    }
    /// Returns a list of the type parameters which are referenced in this
    /// field's type.
    ///
    /// # Caveat
    ///
    /// If the field contains any macros in type position, all parameters will
    /// be considered bound. This is because we cannot determine which type
    /// parameters are bound by type macros.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     struct A<T, U> {
    ///         a: Option<T>,
    ///         b: U,
    ///     }
    /// };
    /// let mut s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].bindings()[0].referenced_ty_params(),
    ///     &[&quote::format_ident!("T")]
    /// );
    /// ```
    pub fn referenced_ty_params(&self) -> Vec<&'a Ident> {
        let mut flags = Vec::new();
        for binding in &self.bindings {
            generics_fuse(&mut flags, &binding.seen_generics);
        }
        fetch_generics(&flags, self.generics)
    }
}
