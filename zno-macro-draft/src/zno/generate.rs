use crate::zno::Quotable;
use crate::zno::Quotables;
use crate::zno::Quote;

/// Generates a `proc_macro2::TokenStream` to be returned by the proc-macro.
///
/// The "Lower" stage is responsible for turning the "Model" data into a `syn::Item` that can be reported here. Currently this is straight forward. However, if additional attribute features are implemented there will likely be some additional complexity here that will help to cut down compile times.
///
/// # Examples
///
/// ```
/// // Assuming `quotes` is a Quotables<Quotable> with at least one Quotable::Item
/// let token_stream = generate(quotes);
/// assert!(token_stream.to_string().starts_with("fn"));
/// ```
///
/// # Panics
///
/// This function will panic if `quotes` does not contain at least one `Quotable::Item`.
///
/// # Arguments
///
/// `quotes` - A `Quotables<Quotable>` object. This should contain at least one `Quotable::Item`.
pub fn generate(quotes: Quotables<Quotable>) -> proc_macro2::TokenStream {
    // Have a logic check earlier to error if there is not **at least one**
    // `Quotable::Item`
    #[allow(clippy::collapsible_match)]
    #[allow(unreachable_patterns)]
    let ts: proc_macro2::TokenStream = match quotes.get(0).expect("An item") {
        Quotable::Item(Quote {
            attrs,
            vis,
            constness,
            unsafety,
            abi,
            ident,
            gen_params,
            params,
            return_type,
            where_clause,
            func_body,
        }) => quote::quote!(
            #(#attrs) *
            #vis #constness #unsafety #abi fn #ident<#gen_params>(#params) #return_type
            #where_clause
            {
                #func_body
            }
        ),
        _ => {
            quote::quote!()
        }
    };
    ts
}

#[cfg(test)]
mod tests {
    use test_utilities::*;

    // Remove #[should_panic] when issue #141 is resolved. Note the integration
    // test for issue #141, which will move to the regression suite when
    // resolved, is blocked by issue #137.  In turn issue #137 is blocked by
    // [macrotest issue 74](https://github.com/eupn/macrotest/issues/74). This
    // in turn appears to be due to [Cargo issue
    // #4942](https://github.com/rust-lang/cargo/issues/4942).  Consequently,
    // depending on whether Cargo resolve this issue or declare it a 'feature'
    // it is possible that the workaround described
    // [here](https://github.com/rust-lang/cargo/issues/4942#issuecomment-357729844)
    // could be a fix.
    //
    // If that is not a fix, the next step is to reorganise the workspace from
    // 'virtual' to 'real' - which requires moving sources around....
    #[test]
    #[should_panic]
    fn generate_1() {
        let i: syn::ItemFn = syn::parse_quote!(
            fn f() {}
        );
        let ts = quote::ToTokens::into_token_stream(i);
        let zno = crate::zno::Zno {
            ..Default::default()
        };

        let models = crate::zno::analyze(zno, ts);

        let quotes = crate::zno::lower(models);
        let rust = crate::zno::generate(quotes);
        let t: syn::ItemFn = syn::parse_quote!(
            fn f() {
                let __guard = zno::local::LocalSpan::enter_with_local_parent("f");
                {}
            }
        );
        let ts: proc_macro2::TokenStream = quote::ToTokens::into_token_stream(t);
        let expected = format!("{:#?}", ts);
        let actual = format!("{:#?}", rust);
        assert_eq_text!(&expected, &actual);
    }
}
