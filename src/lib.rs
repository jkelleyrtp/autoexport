use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Item, ItemMod, Result,
};

/// Derive props for a component within the component definition.
///
/// This macro provides a simple transformation from `Scope<{}>` to `Scope<P>`,
/// removing some boilerplate when defining props.
///
/// You don't *need* to use this macro at all, but it can be helpful in cases where
/// you would be repeating a lot of the usual Rust boilerplate.
///
/// # Example
/// ```ignore
/// #[inline_props]
/// fn app(cx: Scope, bob: String) -> Element {
///     cx.render(rsx!("hello, {bob}"))
/// }
///
/// // is equivalent to
///
/// #[derive(PartialEq, Props)]
/// struct AppProps {
///     bob: String,
/// }
///
/// fn app(cx: Scope<AppProps>) -> Element {
///     cx.render(rsx!("hello, {bob}"))
/// }
/// ```
#[proc_macro_attribute]
pub fn export(_args: proc_macro::TokenStream, s: TokenStream) -> TokenStream {
    match syn::parse::<ExportMod>(s) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}

struct ExportMod {
    item: ItemMod,
}

impl Parse for ExportMod {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ExportMod {
            item: input.parse()?,
        })
    }
}

impl ToTokens for ExportMod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        write_module_to_stream(&self.item, tokens);
    }
}

fn write_module_to_stream(item: &ItemMod, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(&item.attrs);
    tokens.append_all(Some(&item.vis));
    tokens.append_all(Some(&item.mod_token));
    tokens.append_all(Some(&item.ident));

    if let Some((brace, items)) = &item.content {
        brace.surround(tokens, |inner_stream| {
            for inner_item in items {
                match inner_item {
                    Item::Mod(m) => write_module_to_stream(m, inner_stream),
                    other => inner_stream.append_all(quote! { #other }),
                }
            }
        });
    }

    tokens.append_all(item.semi);

    let name = &item.ident;
    tokens.append_all(Some(&item.vis));
    tokens.append_all(quote! { use #name::*; });
}
