use crate::zno::lower::TracedItem;

use syn::spanned::Spanned;

/// Instruments a block of code.
///
/// This function generates the instrumented version of the given block. If the block is part of an async function,
/// it will be wrapped in an async block. Otherwise, the span will be entered and then the rest of the body will be performed.
///
/// # Examples
///
/// ```
/// // Assuming `block` is a syn::Block for the block `{ let x = 5; }`
/// // and `traced_item` is a TracedItem with name "my_func" and `enter_on_poll` set to false
/// let instrumented_block = gen_block(&block, false, traced_item);
/// ```
///
/// # Errors
///
/// This function will return an error if `enter_on_poll` is true but the function is not async.
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
/// `'a` - The lifetime of the reference to the `syn::Block` in the `block` argument.
///
/// # Arguments
///
/// `block` - A reference to a `syn::Block` that should be instrumented.
///
/// `async_context` - A boolean indicating whether the block is part of an async function.
///
/// `traced_item` - A `TracedItem` containing the name of the span and whether it should be entered on poll.
///
/// # Notes
///
/// The function generates the instrumented function body. If the function is an `async fn`, this will wrap it in an async block.
/// Otherwise, this will enter the span and then perform the rest of the body.
pub fn gen_block(
    block: &syn::Block,
    async_context: bool,
    traced_item: TracedItem,
) -> proc_macro2::TokenStream {
    let event = traced_item.name.value();

    if async_context {
        gen_async_block(block, traced_item.enter_on_poll.value, &event)
    } else {
        gen_sync_block(block, traced_item.enter_on_poll.value, &event)
    }
}

/// Generates an async block of code for tracing.
///
/// This function decides whether to generate an async block that enters on poll or not based on the `enter_on_poll` argument.
///
/// # Arguments
///
/// * `block` - A reference to the block of code to generate.
/// * `enter_on_poll` - A boolean indicating whether to enter on poll or not.
/// * `event` - The event to zno.
///
/// # Examples
///
/// ```
/// use syn::parse_quote;
/// use proc_macro2::TokenStream;
///
/// let block: syn::Block = parse_quote! {
///     println!("Hello, world!");
/// };
/// let enter_on_poll = true;
/// let event = "my_event";
/// let token_stream = gen_async_block(&block, enter_on_poll, event);
/// ```
///

fn gen_async_block(
    block: &syn::Block,
    enter_on_poll: bool,
    event: &str,
) -> proc_macro2::TokenStream {
    if enter_on_poll {
        gen_async_block_enter_on_poll(block, event)
    } else {
        gen_async_block_in_span(block, event)
    }
}

/// Generates an async block of code that enters on poll for tracing.
///
/// # Arguments
///
/// * `block` - A reference to the block of code to generate.
/// * `event` - The event to zno.
///
/// # Examples
///
/// ```
/// use syn::parse_quote;
/// use proc_macro2::TokenStream;
///
/// let block: syn::Block = parse_quote! {
///     println!("Hello, world!");
/// };
/// let event = "my_event";
/// let token_stream = gen_async_block_enter_on_poll(&block, event);
/// ```
///

fn gen_async_block_enter_on_poll(
    block: &syn::Block,
    event: &str,
) -> proc_macro2::TokenStream {
    quote::quote_spanned!(block.span()=>
        zno::future::FutureExt::enter_on_poll(
            async move { #block },
            #event
        )
    )
}

/// Generates an async block of code that is in a span for tracing.
///
/// # Arguments
///
/// * `block` - A reference to the block of code to generate.
/// * `event` - The event to zno.
///
/// # Examples
///
/// ```
/// use syn::parse_quote;
/// use proc_macro2::TokenStream;
///
/// let block: syn::Block = parse_quote! {
///     println!("Hello, world!");
/// };
/// let event = "my_event";
/// let token_stream = gen_async_block_in_span(&block, event);
/// ```
///

fn gen_async_block_in_span(
    block: &syn::Block,
    event: &str,
) -> proc_macro2::TokenStream {
    quote::quote_spanned!(block.span()=>
        zno::future::FutureExt::in_span(
            async move { #block },
            zno::Span::enter_with_local_parent( #event )
        )
    )
}

/// Generates a synchronous block of code for tracing.
///
/// This function decides whether to generate a synchronous block that enters on poll or not based on the `enter_on_poll` argument.
///
/// # Arguments
///
/// * `block` - A reference to the block of code to generate.
/// * `enter_on_poll` - A boolean indicating whether to enter on poll or not.
/// * `event` - The event to zno.
///
/// # Examples
///
/// ```
/// use syn::parse_quote;
/// use proc_macro2::TokenStream;
///
/// let block: syn::Block = parse_quote! {
///     println!("Hello, world!");
/// };
/// let enter_on_poll = true;
/// let event = "my_event";
/// let token_stream = gen_sync_block(&block, enter_on_poll, event);
/// ```
fn gen_sync_block(
    block: &syn::Block,
    enter_on_poll: bool,
    event: &str,
) -> proc_macro2::TokenStream {
    if enter_on_poll {
        gen_sync_block_enter_on_poll(block, event)
    } else {
        gen_sync_block_in_span(block, event)
    }
}

/// Generates a synchronous block of code that enters on poll for tracing.
///
/// # Arguments
///
/// * `block` - A reference to the block of code to generate.
/// * `event` - The event to zno.
///
/// # Examples
///
/// ```
/// use syn::parse_quote;
/// use proc_macro2::TokenStream;
///
/// let block: syn::Block = parse_quote! {
///     println!("Hello, world!");
/// };
/// let event = "my_event";
/// let token_stream = gen_sync_block_enter_on_poll(&block, event);
/// ```
///
/// # Errors
///
/// This function will return an error if `enter_on_poll` is applied on a non-async function.
///
fn gen_sync_block_enter_on_poll(
    block: &syn::Block,
    event: &str,
) -> proc_macro2::TokenStream {
    let e = syn::Error::new(
        syn::spanned::Spanned::span(&enter_on_poll),
        "`enter_on_poll` can not be applied on non-async function",
    );
    let tokens = quote::quote_spanned!(block.span()=>
        let __guard = zno::local::LocalSpan::enter_with_local_parent( #event );
        #block
    );
    crate::token_stream_with_error(tokens, e)
}

/// Generates a synchronous block of code that is in a span for tracing.
///
/// # Arguments
///
/// * `block` - A reference to the block of code to generate.
/// * `event` - The event to zno.
///
/// # Examples
///
/// ```
/// use syn::parse_quote;
/// use proc_macro2::TokenStream;
///
/// let block: syn::Block = parse_quote! {
///     println!("Hello, world!");
/// };
/// let event = "my_event";
/// let token_stream = gen_sync_block_in_span(&block, event);
/// ```
fn gen_sync_block_in_span(
    block: &syn::Block,
    event: &str,
) -> proc_macro2::TokenStream {
    quote::quote_spanned!(block.span()=>
        let __guard = zno::local::LocalSpan::enter_with_local_parent( #event );
        #block
    )
}
