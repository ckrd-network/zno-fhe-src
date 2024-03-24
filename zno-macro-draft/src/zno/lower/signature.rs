use crate::zno::lower::lifetime::*;

use syn::visit_mut::VisitMut;

/// Transforms a function signature for async tracing.
///
/// This function modifies the provided function signature to enable async tracing. It adjusts the lifetimes, adds necessary bounds, and changes the return type to a Future.
///
/// # Arguments
///
/// * `sig` - The function signature to transform.
/// * `has_self` - A boolean indicating whether the function has a self parameter.
/// * `is_local` - A boolean indicating whether the function is local.
///
/// # Examples
///
/// ```
/// // Assuming `sig` is a mutable reference to a `syn::Signature` instance, `has_self` and `is_local` are booleans
/// transform_sig(&mut sig, has_self, is_local);
/// ```
///
/// # Safety
///
/// This function does not use any unsafe code.
///
/// # Panics
///
/// This function does not panic under normal conditions.
///
/// # Lifetimes
///
/// This function collects all lifetimes from the function signature and adjusts them for async tracing.
///
pub fn transform_sig(sig: &mut syn::Signature, has_self: bool, is_local: bool) {

    update_span(sig);

    let ret = get_return_type(sig);
    let default_span = get_default_span(sig);
    let mut lifetimes = collect_lifetimes(sig, default_span);

    update_generics(sig, &mut lifetimes, default_span);

    update_lifetimes(sig, &mut lifetimes, default_span);

    update_self_bounds(sig, has_self, is_local);

    update_inputs(sig);

    update_output(sig, ret, is_local);
}

/// Updates the span of the function token to match the span of the asyncness token.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
///
/// # Panics
///
/// This function will panic if the asyncness token is not present in the function's signature.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// update_span(&mut sig);
/// ```
fn update_span(sig: &mut syn::Signature) {
    sig.fn_token.span = sig.asyncness.take().unwrap().span;
}

/// Returns the return type of the function as a token stream.
///
/// # Arguments
///
/// * `sig` - A reference to the function's signature.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned, ReturnType};
/// use proc_macro2::Span;
///
/// let sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     output: ReturnType::Default,
///     ..Signature::default()
/// };
/// let return_type = get_return_type(&sig);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn get_return_type(sig: &syn::Signature) -> proc_macro2::TokenStream {
    match &sig.output {
        syn::ReturnType::Default => quote::quote!(()),
        syn::ReturnType::Type(_, ret) => quote::quote!(#ret),
    }
}

/// Returns the joined span of the function's identifier and parentheses token, or the span of the identifier if the join fails.
///
/// # Arguments
///
/// * `sig` - A reference to the function's signature.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let default_span = get_default_span(&sig);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn get_default_span(sig: &syn::Signature) -> proc_macro2::Span {
    sig.ident
        .span()
        .join(sig.paren_token.span)
        .unwrap_or_else(|| sig.ident.span())
}

/// Collects the lifetimes from the function's inputs and returns a `CollectLifetimes` instance.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `default_span` - The default span to use for the collected lifetimes.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let default_span = get_default_span(&sig);
/// let lifetimes = collect_lifetimes(&mut sig, default_span);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn collect_lifetimes(sig: &mut syn::Signature, default_span: proc_macro2::Span) -> CollectLifetimes {
    let mut lifetimes = CollectLifetimes::new("'life", default_span);
    for arg in sig.inputs.iter_mut() {
        match arg {
            syn::FnArg::Receiver(arg) => lifetimes.visit_receiver_mut(arg),
            syn::FnArg::Typed(arg) => lifetimes.visit_type_mut(&mut arg.ty),
        }
    }
    lifetimes
}

/// Updates the generics of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `lifetimes` - A reference to the collected lifetimes.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let lifetimes = CollectLifetimes::new("'life", Span::call_site());
/// update_generics(&mut sig, &lifetimes);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_generics(sig: &mut syn::Signature, lifetimes: &mut CollectLifetimes) {
    update_generic_params(sig);
    update_generic_tokens(sig);
}

/// Updates the generic parameters of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// update_generic_params(&mut sig);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_generic_params(sig: &mut syn::Signature) {
    for param in sig.generics.params.iter() {
        match param {
            syn::GenericParam::Type(param) => update_type_param(sig, param),
            syn::GenericParam::Lifetime(param) => update_lifetime_param(sig, param),
            syn::GenericParam::Const(_) => {}
        }
    }
}

/// Updates a type parameter in a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `param` - A reference to the type parameter to update.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, TypeParam, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let param = TypeParam {
///     ident: Ident::new("T", Span::call_site()),
///     ..TypeParam::default()
/// };
/// update_type_param(&mut sig, &param);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_type_param(sig: &mut syn::Signature, param: &syn::TypeParam) {
    let param = &param.ident;
    let span = param.span();
    where_clause_or_default(&mut sig.generics.where_clause)
        .predicates
        .push(syn::parse_quote_spanned!(span=> #param: 'minitrace));
}

/// Updates a lifetime parameter in a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `param` - A reference to the lifetime parameter to update.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, LifetimeDef, Lifetime, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let param = LifetimeDef {
///     lifetime: Lifetime::new("'a", Span::call_site()),
///     ..LifetimeDef::default()
/// };
/// update_lifetime_param(&mut sig, &param);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_lifetime_param(sig: &mut syn::Signature, param: &syn::LifetimeDef) {
    let param = &param.lifetime;
    let span = param.span();
    where_clause_or_default(&mut sig.generics.where_clause)
        .predicates
        .push(syn::parse_quote_spanned!(span=> #param: 'minitrace));
}
/// Updates the generic tokens of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// update_generic_tokens(&mut sig);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_generic_tokens(sig: &mut syn::Signature) {
    if sig.generics.lt_token.is_none() {
        sig.generics.lt_token = Some(syn::Token![<](sig.ident.span()));
    }
    if sig.generics.gt_token.is_none() {
        sig.generics.gt_token = Some(syn::Token![>](sig.paren_token.span));
    }
}

/// Updates the lifetimes of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `lifetimes` - A mutable reference to the collected lifetimes.
/// * `default_span` - The default span to use for the updated lifetimes.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let mut lifetimes = CollectLifetimes::new("'life", Span::call_site());
/// let default_span = get_default_span(&sig);
/// update_lifetimes(&mut sig, &mut lifetimes, default_span);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_lifetimes(sig: &mut syn::Signature, lifetimes: &mut CollectLifetimes, default_span: proc_macro2::Span) {
    update_named_lifetimes(sig, &lifetimes.named);
    update_elided_lifetimes(sig, &lifetimes.elided);
    insert_minitrace_lifetime(sig, default_span);
}
/// Updates the named lifetimes of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `named_lifetimes` - A reference to the vector of named lifetimes.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, Lifetime, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let named_lifetimes = vec![Lifetime::new("'a", Span::call_site())];
/// update_named_lifetimes(&mut sig, &named_lifetimes);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_named_lifetimes(sig: &mut syn::Signature, named_lifetimes: &Vec<syn::Lifetime>) {
    for (idx, lifetime) in named_lifetimes.iter().enumerate() {
        sig.generics.params.insert(idx, syn::parse_quote!(#lifetime));
        where_clause_or_default(&mut sig.generics.where_clause)
            .predicates
            .push(syn::parse_quote_spanned!(lifetime.span()=> #lifetime: 'minitrace));
    }
}

/// Updates the elided lifetimes of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `elided_lifetimes` - A reference to the vector of elided lifetimes.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, Lifetime, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let elided_lifetimes = vec![Lifetime::new("'_", Span::call_site())];
/// update_elided_lifetimes(&mut sig, &elided_lifetimes);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_elided_lifetimes(sig: &mut syn::Signature, elided_lifetimes: &Vec<syn::Lifetime>) {
    for (idx, elided) in elided_lifetimes.iter().enumerate() {
        sig.generics.params.insert(idx, syn::parse_quote!(#elided));
        where_clause_or_default(&mut sig.generics.where_clause)
            .predicates
            .push(syn::parse_quote_spanned!(elided.span()=> #elided: 'minitrace));
    }
}
/// Inserts a 'minitrace' lifetime at the beginning of the generics of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `default_span` - The default span to use for the 'minitrace' lifetime.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let default_span = get_default_span(&sig);
/// insert_minitrace_lifetime(&mut sig, default_span);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn insert_minitrace_lifetime(sig: &mut syn::Signature, default_span: proc_macro2::Span) {
    sig.generics
        .params
        .insert(0, syn::parse_quote_spanned!(default_span=> 'minitrace));
}

/// Updates the self bounds of a function's signature.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature.
/// * `has_self` - A boolean indicating whether the function has a self parameter.
/// * `is_local` - A boolean indicating whether the function is local.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let has_self = true;
/// let is_local = false;
/// update_self_bounds(&mut sig, has_self, is_local);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn update_self_bounds(sig: &mut syn::Signature, has_self: bool, is_local: bool) {
    if has_self {
        let bound_span = sig.ident.span();
        let bound = determine_bound(sig, bound_span);

        let where_clause = where_clause_or_default(&mut sig.generics.where_clause);
        where_clause.predicates.push(if is_local {
            syn::parse_quote_spanned!(bound_span=> Self: 'minitrace)
        } else {
            syn::parse_quote_spanned!(bound_span=> Self: ::core::marker::#bound + 'minitrace)
        });
    }
}

/// Determines the bound for a given function signature.
///
/// # Arguments
///
/// * `sig` - A reference to the function's signature.
/// * `bound_span` - The span to use for the bound.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let bound_span = get_default_span(&sig);
/// let bound = determine_bound(&sig, bound_span);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn determine_bound(sig: &syn::Signature, bound_span: proc_macro2::Span) -> syn::Ident {
    match get_first_input_type(sig) {
        Some(input_type) => create_ident_based_on_input_type(input_type, bound_span),
        None => syn::Ident::new("Send", bound_span),
    }
}

/// Creates an identifier based on the input type.
///
/// # Arguments
///
/// * `input_type` - The input type to base the identifier on.
/// * `bound_span` - The span to use for the identifier.
///
/// # Examples
///
/// ```
/// use syn::{Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let input_type = InputType::Receiver;
/// let bound_span = Span::call_site();
/// let ident = create_ident_based_on_input_type(input_type, bound_span);
/// ```
///
/// # Panics
///
/// This function does not panic.
fn create_ident_based_on_input_type(input_type: InputType, bound_span: proc_macro2::Span) -> syn::Ident {
    match input_type {
        InputType::Receiver => create_sync_ident(bound_span),
        InputType::Typed(pat_ident, ty_mutability) => {
            if is_sync_type(&pat_ident, &ty_mutability) {
                create_sync_ident(bound_span)
            } else {
                create_send_ident(bound_span)
            }
        }
    }
}

/// Creates a new `Sync` identifier with the given span.
///
/// # Arguments
///
/// * `bound_span` - The span to use for the identifier.
///
/// # Examples
///
/// ```
/// use proc_macro2::Span;
///
/// let bound_span = Span::call_site();
/// let ident = create_sync_ident(bound_span);
/// ```
///

fn create_sync_ident(bound_span: proc_macro2::Span) -> syn::Ident {
    syn::Ident::new("Sync", bound_span)
}

/// Creates a new `Send` identifier with the given span.
///
/// # Arguments
///
/// * `bound_span` - The span to use for the identifier.
///
/// # Examples
///
/// ```
/// use proc_macro2::Span;
///
/// let bound_span = Span::call_site();
/// let ident = create_send_ident(bound_span);
/// ```
///

fn create_send_ident(bound_span: proc_macro2::Span) -> syn::Ident {
    syn::Ident::new("Send", bound_span)
}

/// Determines if the given identifier and mutability represent a `Sync` type.
///
/// # Arguments
///
/// * `pat_ident` - The identifier to check.
/// * `ty_mutability` - The mutability to check.
///
/// # Examples
///
/// ```
/// let pat_ident = "self";
/// let ty_mutability = None;
/// let is_sync = is_sync_type(pat_ident, &ty_mutability);
/// ```
///

fn is_sync_type(pat_ident: &str, ty_mutability: &Option<syn::Mutability>) -> bool {
    pat_ident == "self" && ty_mutability.is_none()
}

/// Represents the type of an input to a function.
///
/// # Variants
///
/// * `Receiver` - The input is a receiver.
/// * `Typed` - The input is a typed argument, with a string representing the type and an optional mutability.
enum InputType {
    Receiver,
    Typed(String, Option<syn::Mutability>),
}
fn get_first_input_type(sig: &syn::Signature) -> Option<InputType> {
    match sig.inputs.iter().next() {
        Some(syn::FnArg::Receiver(receiver)) => handle_receiver(receiver),
        Some(syn::FnArg::Typed(arg)) => handle_typed_arg(arg),
        _ => None,
    }
}

fn handle_receiver(receiver: &syn::Receiver) -> Option<InputType> {
    match receiver {
        syn::Receiver {
            reference: Some(_),
            mutability: None,
            ..
        } => Some(InputType::Receiver),
        _ => None,
    }
}

fn handle_typed_arg(arg: &syn::PatType) -> Option<InputType> {
    match (arg.pat.as_ref(), arg.ty.as_ref()) {
        (syn::Pat::Ident(pat), syn::Type::Reference(ty)) => {
            Some(InputType::Typed(pat.ident.to_string(), ty.mutability.clone()))
        }
        _ => None,
    }
}

/// Updates the inputs of a given function signature.
///
/// This function iterates over the inputs of the function signature and updates them based on their type.
/// If the input is a receiver with a reference, it does nothing.
/// If the input is a receiver without a reference, it removes its mutability.
/// If the input is a typed argument, it removes the reference and mutability from the identifier, if present.
/// Otherwise, it replaces the pattern with a new one that has a positional argument and an optional mutability.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature to update.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// update_inputs(&mut sig);
/// ```
///

fn update_inputs(sig: &mut syn::Signature) {
    for (i, arg) in sig.inputs.iter_mut().enumerate() {
        match arg {
            syn::FnArg::Receiver(receiver) => update_receiver(receiver),
            syn::FnArg::Typed(arg) => update_typed_arg(i, arg),
        }
    }
}
/// Updates a receiver argument in a function signature.
///
/// If the receiver has a reference, it removes its mutability.
///
/// # Arguments
///
/// * `receiver` - A mutable reference to the receiver to update.
///
/// # Examples
///
/// ```
/// use syn::Receiver;
///
/// let mut receiver = Receiver {
///     attrs: Vec::new(),
///     reference: Some(Default::default()),
///     mutability: Some(Default::default()),
///     self_token: Default::default(),
/// };
/// update_receiver(&mut receiver);
/// ```
///

fn update_receiver(receiver: &mut syn::Receiver) {
    if receiver.reference.is_some() {
        receiver.mutability = None;
    }
}

/// Updates a typed argument in a function signature.
///
/// If the argument is an identifier, it removes the reference from the identifier.
/// Otherwise, it replaces the pattern with a new one that has a positional argument and an optional mutability.
///
/// # Arguments
///
/// * `i` - The index of the argument in the function signature.
/// * `arg` - A mutable reference to the typed argument to update.
///
/// # Examples
///
/// ```
/// use syn::{PatType, Ident, Type, spanned::Spanned};
/// use proc_macro2::Span;
///
/// let mut arg = PatType {
///     attrs: Vec::new(),
///     pat: Box::new(Pat::Ident(Ident::new("foo", Span::call_site()))),
///     colon_token: Default::default(),
///     ty: Box::new(Type::Verbatim(Default::default())),
/// };
/// update_typed_arg(0, &mut arg);
/// ```
///

fn update_typed_arg(i: usize, arg: &mut syn::PatType) {
    if let syn::Pat::Ident(ident) = &mut *arg.pat {
        ident.by_ref = None;
        //ident.mutability = None;
    } else {
        let positional = positional_arg(i, &arg.pat);
        let m = mut_pat(&mut arg.pat);
        arg.pat = syn::parse_quote!(#m #positional);
    }
}

/// Updates the output of a given function signature.
///
/// This function replaces the output of the function signature with a new one that returns a future with the specified return type and bounds.
///
/// # Arguments
///
/// * `sig` - A mutable reference to the function's signature to update.
/// * `ret` - The return type to use for the future.
/// * `is_local` - A boolean indicating whether the future is local.
///
/// # Examples
///
/// ```
/// use syn::{Signature, Ident, spanned::Spanned};
/// use proc_macro2::{Span, TokenStream};
///
/// let mut sig = Signature {
///     ident: Ident::new("foo", Span::call_site()),
///     ..Signature::default()
/// };
/// let ret = TokenStream::new();
/// update_output(&mut sig, ret, true);
/// ```
///

fn update_output(sig: &mut syn::Signature, ret: proc_macro2::TokenStream, is_local: bool) {
    let ret_span = sig.ident.span();
    let bounds = if is_local {
        quote::quote_spanned!(ret_span=> 'minitrace)
    } else {
        quote::quote_spanned!(ret_span=> ::core::marker::Send + 'minitrace)
    };
    sig.output = syn::parse_quote_spanned! {ret_span=>
        -> impl ::core::future::Future<Output = #ret> + #bounds
    };
}

/// Generates an identifier for a positional argument.
///
/// # Examples
///
/// ```
/// // Assuming `pat` is a `syn::Pat` instance
/// let arg_ident = positional_arg(1, &pat);
/// ```
///
/// # Safety
///
/// This function does not use any unsafe code.
///
/// # Arguments
///
/// * `i` - The index of the argument.
/// * `pat` - The pattern of the argument.
///
/// # Notes
///
/// The `positional_arg` function is used to generate an identifier for a positional argument. It uses the `quote::format_ident!` macro to generate the identifier.
fn positional_arg(i: usize, pat: &syn::Pat) -> syn::Ident {
    quote::format_ident!("__arg{}", i, span = syn::spanned::Spanned::span(&pat))
}

/// Checks if a pattern is mutable.
///
/// # Examples
///
/// ```
/// // Assuming `pat` is a mutable `syn::Pat` instance
/// let is_mut = mut_pat(&mut pat);
/// ```
///
/// # Safety
///
/// This function does not use any unsafe code.
///
/// # Arguments
///
/// `pat` - The pattern to check.
///
/// # Notes
///
/// The `mut_pat` function is used to check if a pattern is mutable. It uses a visitor pattern to traverse the pattern and check for mutability.
fn mut_pat(pat: &mut syn::Pat) -> Option<syn::Token![mut]> {
    let mut visitor = HasMutPat(None);
    visitor.visit_pat_mut(pat);
    visitor.0
}

/// Checks if the `Self` keyword is present in a token stream.
///
/// # Examples
///
/// ```
/// // Assuming `tokens` is a `proc_macro2::TokenStream` instance
/// let has_self = has_self_in_token_stream(tokens);
/// ```
///
/// # Safety
///
/// This function does not use any unsafe code.
///
/// # Arguments
///
/// `tokens` - The token stream to check.
///
/// # Notes
///
/// The `has_self_in_token_stream` function is used to check if the `Self` keyword is present in a token stream. It uses a recursive approach to traverse the token stream.
fn has_self_in_token_stream(tokens: proc_macro2::TokenStream) -> bool {
    tokens.into_iter().any(|tt| match tt {
        proc_macro2::TokenTree::Ident(ident) => ident == "Self",
        proc_macro2::TokenTree::Group(group) => has_self_in_token_stream(group.stream()),
        _ => false,
    })
}

/// Returns the `WhereClause` of a function, or creates a new one if it doesn't exist.
///
/// # Examples
///
/// ```
/// // Assuming `clause` is a mutable reference to an `Option<syn::WhereClause>` instance
/// let where_clause = where_clause_or_default(&mut clause);
/// ```
///
/// # Safety
///
/// This function does not use any unsafe code.
///
/// # Arguments
///
/// `clause` - The `WhereClause` of the function.
///
/// # Notes
///
/// The `where_clause_or_default` function is used to get the `WhereClause` of a function, or create a new one if it doesn't exist. It uses the `Option::get_or_insert_with` method to achieve this.
fn where_clause_or_default(clause: &mut Option<syn::WhereClause>) -> &mut syn::WhereClause {
    clause.get_or_insert_with(|| syn::WhereClause {
        where_token: Default::default(),
        predicates: syn::punctuated::Punctuated::new(),
    })
}

struct HasMutPat(Option<syn::Token![mut]>);

impl syn::visit_mut::VisitMut for HasMutPat {
    fn visit_pat_ident_mut(&mut self, i: &mut syn::PatIdent) {
        if let Some(m) = i.mutability {
            self.0 = Some(m);
        } else {
            syn::visit_mut::visit_pat_ident_mut(self, i);
        }
    }
}

pub struct HasSelf(pub bool);

impl syn::visit_mut::VisitMut for HasSelf {
    /// Visits the `ExprPath` nodes in the syntax tree.
    ///
    /// This method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the first segment of the path is `Self` and updates the state accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `visitor` is a `HasSelf` instance and `expr` is a `syn::ExprPath` instance
    /// visitor.visit_expr_path_mut(&mut expr);
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not use any unsafe code.
    ///
    /// # Arguments
    ///
    /// `expr` - The `ExprPath` node to visit.
    ///
    /// # Notes
    ///
    /// The `visit_expr_path_mut` method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the first segment of the path is `Self` and updates the state accordingly.
    fn visit_expr_path_mut(&mut self, expr: &mut syn::ExprPath) {
        self.0 |= expr.path.segments[0].ident == "Self";
        syn::visit_mut::visit_expr_path_mut(self, expr);
    }

    /// Visits the `PatPath` nodes in the syntax tree.
    ///
    /// This method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the first segment of the path is `Self` and updates the state accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `visitor` is a `HasSelf` instance and `pat` is a `syn::PatPath` instance
    /// visitor.visit_pat_path_mut(&mut pat);
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not use any unsafe code.
    ///
    /// # Arguments
    ///
    /// `pat` - The `PatPath` node to visit.
    ///
    /// # Notes
    ///
    /// The `visit_pat_path_mut` method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the first segment of the path is `Self` and updates the state accordingly.
    fn visit_pat_path_mut(&mut self, pat: &mut syn::PatPath) {
        self.0 |= pat.path.segments[0].ident == "Self";
        syn::visit_mut::visit_pat_path_mut(self, pat);
    }

    /// Visits the `TypePath` nodes in the syntax tree.
    ///
    /// This method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the first segment of the path is `Self` and updates the state accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `visitor` is a `HasSelf` instance and `ty` is a `syn::TypePath` instance
    /// visitor.visit_type_path_mut(&mut ty);
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not use any unsafe code.
    ///
    /// # Arguments
    ///
    /// `ty` - The `TypePath` node to visit.
    ///
    /// # Notes
    ///
    /// The `visit_type_path_mut` method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the first segment of the path is `Self` and updates the state accordingly.
    fn visit_type_path_mut(&mut self, ty: &mut syn::TypePath) {
        self.0 |= ty.path.segments[0].ident == "Self";
        syn::visit_mut::visit_type_path_mut(self, ty);
    }

    /// Visits the `Receiver` nodes in the syntax tree.
    ///
    /// This method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It sets the state to `true` when a `Receiver` node is visited.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `visitor` is a `HasSelf` instance and `arg` is a `syn::Receiver` instance
    /// visitor.visit_receiver_mut(&mut arg);
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not use any unsafe code.
    ///
    /// # Arguments
    ///
    /// `arg` - The `Receiver` node to visit.
    ///
    /// # Notes
    ///
    /// The `visit_receiver_mut` method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It sets the state to `true` when a `Receiver` node is visited.
    fn visit_receiver_mut(&mut self, _arg: &mut syn::Receiver) {
        self.0 = true;
    }

    /// Visits the `Item` nodes in the syntax tree.
    ///
    /// This method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It does not recurse into nested items.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `visitor` is a `HasSelf` instance and `item` is a `syn::Item` instance
    /// visitor.visit_item_mut(&mut item);
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not use any unsafe code.
    ///
    /// # Arguments
    ///
    /// `item` - The `Item` node to visit.
    ///
    /// # Notes
    ///
    /// The `visit_item_mut` method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It does not recurse into nested items.
    fn visit_item_mut(&mut self, _: &mut syn::Item) {
        // Do not recurse into nested items.
    }

    /// Visits the `Macro` nodes in the syntax tree.
    ///
    /// This method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the macro contains a function and updates the state accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `visitor` is a `HasSelf` instance and `mac` is a `syn::Macro` instance
    /// visitor.visit_macro_mut(&mut mac);
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not use any unsafe code.
    ///
    /// # Arguments
    ///
    /// `mac` - The `Macro` node to visit.
    ///
    /// # Notes
    ///
    /// The `visit_macro_mut` method is part of the `syn::visit_mut::VisitMut` trait implementation for `HasSelf`. It checks if the macro contains a function and updates the state accordingly.
    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        if !contains_fn(mac.tokens.clone()) {
            self.0 |= has_self_in_token_stream(mac.tokens.clone());
        }
    }
}

/// Checks if the `fn` keyword is present in a token stream.
///
/// This function is used to check if the `fn` keyword is present in a token stream. It uses a recursive approach to traverse the token stream.
///
/// # Examples
///
/// ```
/// // Assuming `tokens` is a `proc_macro2::TokenStream` instance
/// let contains_fn = contains_fn(tokens);
/// ```
///
/// # Safety
///
/// This function does not use any unsafe code.
///
/// # Arguments
///
/// `tokens` - The token stream to check.
///
/// # Notes
///
/// The `contains_fn` function is used to check if the `fn` keyword is present in a token stream. It uses a recursive approach to traverse the token stream.
fn contains_fn(tokens: proc_macro2::TokenStream) -> bool {
    tokens.into_iter().any(|tt| match tt {
        proc_macro2::TokenTree::Ident(ident) => ident == "fn",
        proc_macro2::TokenTree::Group(group) => contains_fn(group.stream()),
        _ => false,
    })
}
