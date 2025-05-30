use proc_macro::TokenStream;
use quote::{ToTokens as _, quote};
use syn::{parse, *};

#[proc_macro_attribute]
pub fn slow_warning(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed = parse(item).unwrap();

    let Item::Fn(function) = parsed else {
        panic!("This can only be used on functions");
    };

    let block = Block {
        brace_token: function.block.brace_token,
        stmts: vec![],
    };

    let mut wrapped_function = ItemFn {
        attrs: function.attrs.clone(),
        vis: function.vis.clone(),
        sig: function.sig.clone(),
        block: Box::new(block),
    };

    let block = function.block;

    let quoted = quote! {{
        let closure = move || {
            #block
        };

        let mut result = closure();

        result
    }};

    let token_stream = quoted.into();
    let parsed = parse(token_stream).unwrap();

    wrapped_function.block = parsed;

    let token_stream = wrapped_function.into_token_stream();
    token_stream.into()
}
