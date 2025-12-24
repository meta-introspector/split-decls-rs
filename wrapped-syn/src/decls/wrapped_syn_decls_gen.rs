use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[rustfmt::skip]
mod gen {
    /// Syntax tree traversal to transform the nodes of an owned syntax tree.
    ///
    /// Each method of the [`Fold`] trait is a hook that can be overridden to
    /// customize the behavior when transforming the corresponding type of node.
    /// By default, every method recursively visits the substructure of the
    /// input by invoking the right visitor method of each of its fields.
    ///
    /// [`Fold`]: fold::Fold
    ///
    /// ```
    /// # use syn::{Attribute, BinOp, Expr, ExprBinary};
    /// #
    /// pub trait Fold {
    ///     /* ... */
    ///
    ///     fn fold_expr_binary(&mut self, node: ExprBinary) -> ExprBinary {
    ///         fold_expr_binary(self, node)
    ///     }
    ///
    ///     /* ... */
    ///     # fn fold_attribute(&mut self, node: Attribute) -> Attribute;
    ///     # fn fold_expr(&mut self, node: Expr) -> Expr;
    ///     # fn fold_bin_op(&mut self, node: BinOp) -> BinOp;
    /// }
    ///
    /// pub fn fold_expr_binary<V>(v: &mut V, node: ExprBinary) -> ExprBinary
    /// where
    ///     V: Fold + ?Sized,
    /// {
    ///     ExprBinary {
    ///         attrs: node
    ///             .attrs
    ///             .into_iter()
    ///             .map(|attr| v.fold_attribute(attr))
    ///             .collect(),
    ///         left: Box::new(v.fold_expr(*node.left)),
    ///         op: v.fold_bin_op(node.op),
    ///         right: Box::new(v.fold_expr(*node.right)),
    ///     }
    /// }
    ///
    /// /* ... */
    /// ```
    ///
    /// <br>
    ///
    /// # Example
    ///
    /// This fold inserts parentheses to fully parenthesizes any expression.
    ///
    /// ```
    /// // [dependencies]
    /// // quote = "1.0"
    /// // syn = { version = "2.0", features = ["fold", "full"] }
    ///
    /// use quote::quote;
    /// use syn::fold::{fold_expr, Fold};
    /// use syn::{token, Expr, ExprParen};
    ///
    /// struct ParenthesizeEveryExpr;
    ///
    /// impl Fold for ParenthesizeEveryExpr {
    ///     fn fold_expr(&mut self, expr: Expr) -> Expr {
    ///         Expr::Paren(ExprParen {
    ///             attrs: Vec::new(),
    ///             expr: Box::new(fold_expr(self, expr)),
    ///             paren_token: token::Paren::default(),
    ///         })
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let code = quote! { a() + b(1) * c.d };
    ///     let expr: Expr = syn::parse2(code).unwrap();
    ///     let parenthesized = ParenthesizeEveryExpr.fold_expr(expr);
    ///     println!("{}", quote!(#parenthesized));
    ///
    ///     // Output: (((a)()) + (((b)((1))) * ((c).d)))
    /// }
    /// ```
    #[cfg(feature = "fold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fold")))]
    #[rustfmt::skip]
    pub mod fold;
    /// Syntax tree traversal to walk a shared borrow of a syntax tree.
    ///
    /// Each method of the [`Visit`] trait is a hook that can be overridden to
    /// customize the behavior when visiting the corresponding type of node. By
    /// default, every method recursively visits the substructure of the input
    /// by invoking the right visitor method of each of its fields.
    ///
    /// [`Visit`]: visit::Visit
    ///
    /// ```
    /// # use syn::{Attribute, BinOp, Expr, ExprBinary};
    /// #
    /// pub trait Visit<'ast> {
    ///     /* ... */
    ///
    ///     fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
    ///         visit_expr_binary(self, node);
    ///     }
    ///
    ///     /* ... */
    ///     # fn visit_attribute(&mut self, node: &'ast Attribute);
    ///     # fn visit_expr(&mut self, node: &'ast Expr);
    ///     # fn visit_bin_op(&mut self, node: &'ast BinOp);
    /// }
    ///
    /// pub fn visit_expr_binary<'ast, V>(v: &mut V, node: &'ast ExprBinary)
    /// where
    ///     V: Visit<'ast> + ?Sized,
    /// {
    ///     for attr in &node.attrs {
    ///         v.visit_attribute(attr);
    ///     }
    ///     v.visit_expr(&*node.left);
    ///     v.visit_bin_op(&node.op);
    ///     v.visit_expr(&*node.right);
    /// }
    ///
    /// /* ... */
    /// ```
    ///
    /// <br>
    ///
    /// # Example
    ///
    /// This visitor will print the name of every freestanding function in the
    /// syntax tree, including nested functions.
    ///
    /// ```
    /// // [dependencies]
    /// // quote = "1.0"
    /// // syn = { version = "2.0", features = ["full", "visit"] }
    ///
    /// use quote::quote;
    /// use syn::visit::{self, Visit};
    /// use syn::{File, ItemFn};
    ///
    /// struct FnVisitor;
    ///
    /// impl<'ast> Visit<'ast> for FnVisitor {
    ///     fn visit_item_fn(&mut self, node: &'ast ItemFn) {
    ///         println!("Function with name={}", node.sig.ident);
    ///
    ///         // Delegate to the default impl to visit any nested functions.
    ///         visit::visit_item_fn(self, node);
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let code = quote! {
    ///         pub fn f() {
    ///             fn g() {}
    ///         }
    ///     };
    ///
    ///     let syntax_tree: File = syn::parse2(code).unwrap();
    ///     FnVisitor.visit_file(&syntax_tree);
    /// }
    /// ```
    ///
    /// The `'ast` lifetime on the input references means that the syntax tree
    /// outlives the complete recursive visit call, so the visitor is allowed to
    /// hold on to references into the syntax tree.
    ///
    /// ```
    /// use quote::quote;
    /// use syn::visit::{self, Visit};
    /// use syn::{File, ItemFn};
    ///
    /// struct FnVisitor<'ast> {
    ///     functions: Vec<&'ast ItemFn>,
    /// }
    ///
    /// impl<'ast> Visit<'ast> for FnVisitor<'ast> {
    ///     fn visit_item_fn(&mut self, node: &'ast ItemFn) {
    ///         self.functions.push(node);
    ///         visit::visit_item_fn(self, node);
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let code = quote! {
    ///         pub fn f() {
    ///             fn g() {}
    ///         }
    ///     };
    ///
    ///     let syntax_tree: File = syn::parse2(code).unwrap();
    ///     let mut visitor = FnVisitor { functions: Vec::new() };
    ///     visitor.visit_file(&syntax_tree);
    ///     for f in visitor.functions {
    ///         println!("Function with name={}", f.sig.ident);
    ///     }
    /// }
    /// ```
    #[cfg(feature = "visit")]
    #[cfg_attr(docsrs, doc(cfg(feature = "visit")))]
    #[rustfmt::skip]
    pub mod visit;
    /// Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
    /// place.
    ///
    /// Each method of the [`VisitMut`] trait is a hook that can be overridden
    /// to customize the behavior when mutating the corresponding type of node.
    /// By default, every method recursively visits the substructure of the
    /// input by invoking the right visitor method of each of its fields.
    ///
    /// [`VisitMut`]: visit_mut::VisitMut
    ///
    /// ```
    /// # use syn::{Attribute, BinOp, Expr, ExprBinary};
    /// #
    /// pub trait VisitMut {
    ///     /* ... */
    ///
    ///     fn visit_expr_binary_mut(&mut self, node: &mut ExprBinary) {
    ///         visit_expr_binary_mut(self, node);
    ///     }
    ///
    ///     /* ... */
    ///     # fn visit_attribute_mut(&mut self, node: &mut Attribute);
    ///     # fn visit_expr_mut(&mut self, node: &mut Expr);
    ///     # fn visit_bin_op_mut(&mut self, node: &mut BinOp);
    /// }
    ///
    /// pub fn visit_expr_binary_mut<V>(v: &mut V, node: &mut ExprBinary)
    /// where
    ///     V: VisitMut + ?Sized,
    /// {
    ///     for attr in &mut node.attrs {
    ///         v.visit_attribute_mut(attr);
    ///     }
    ///     v.visit_expr_mut(&mut *node.left);
    ///     v.visit_bin_op_mut(&mut node.op);
    ///     v.visit_expr_mut(&mut *node.right);
    /// }
    ///
    /// /* ... */
    /// ```
    ///
    /// <br>
    ///
    /// # Example
    ///
    /// This mut visitor replace occurrences of u256 suffixed integer literals
    /// like `999u256` with a macro invocation `bigint::u256!(999)`.
    ///
    /// ```
    /// // [dependencies]
    /// // quote = "1.0"
    /// // syn = { version = "2.0", features = ["full", "visit-mut"] }
    ///
    /// use quote::quote;
    /// use syn::visit_mut::{self, VisitMut};
    /// use syn::{parse_quote, Expr, File, Lit, LitInt};
    ///
    /// struct BigintReplace;
    ///
    /// impl VisitMut for BigintReplace {
    ///     fn visit_expr_mut(&mut self, node: &mut Expr) {
    ///         if let Expr::Lit(expr) = &node {
    ///             if let Lit::Int(int) = &expr.lit {
    ///                 if int.suffix() == "u256" {
    ///                     let digits = int.base10_digits();
    ///                     let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
    ///                     *node = parse_quote!(bigint::u256!(#unsuffixed));
    ///                     return;
    ///                 }
    ///             }
    ///         }
    ///
    ///         // Delegate to the default impl to visit nested expressions.
    ///         visit_mut::visit_expr_mut(self, node);
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let code = quote! {
    ///         fn main() {
    ///             let _ = 999u256;
    ///         }
    ///     };
    ///
    ///     let mut syntax_tree: File = syn::parse2(code).unwrap();
    ///     BigintReplace.visit_file_mut(&mut syntax_tree);
    ///     println!("{}", quote!(#syntax_tree));
    /// }
    /// ```
    #[cfg(feature = "visit-mut")]
    #[cfg_attr(docsrs, doc(cfg(feature = "visit-mut")))]
    #[rustfmt::skip]
    pub mod visit_mut;
    #[cfg(feature = "clone-impls")]
    #[rustfmt::skip]
    mod clone;
    #[cfg(feature = "extra-traits")]
    #[rustfmt::skip]
    mod debug;
    #[cfg(feature = "extra-traits")]
    #[rustfmt::skip]
    mod eq;
    #[cfg(feature = "extra-traits")]
    #[rustfmt::skip]
    mod hash;
}
