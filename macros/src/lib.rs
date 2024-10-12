use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn machine(_attrs: TokenStream, fun: TokenStream) -> TokenStream {
    let f = syn::parse_macro_input!(fun as syn::ItemFn);

    assert!(
        f.attrs.is_empty(),
        "netsim_embed::machine cannot have any attribute"
    );
    assert!(
        f.sig.constness.is_none(),
        "netsim_embed::machine cannot be const"
    );
    assert!(
        f.sig.unsafety.is_none(),
        "netsim_embed::machine cannot be unsafe"
    );
    assert!(
        f.sig.abi.is_none(),
        "netsim_embed::machine cannot have an abi defined"
    );
    assert!(
        f.sig.generics.params.is_empty(),
        "netsim_embed::machine cannot be generic"
    );
    assert!(
        f.sig.variadic.is_none(),
        "netsim_embed::machine cannot be variadic"
    );
    assert!(
        matches!(f.sig.output, syn::ReturnType::Default),
        "netsim_embed::machine must not declare a return type"
    );

    let mut inputs = vec![];
    for input in &f.sig.inputs {
        match input {
            syn::FnArg::Typed(input) => {
                assert!(
                    input.attrs.is_empty(),
                    "netsim_embed::machine's only argument must not have any attributes attached"
                );
                inputs.push(input);
            }
            _ => panic!("netsim_embed::machine must be a freestanding function"),
        }
    }

    let f_vis = f.vis;
    let f_ident = f.sig.ident;
    let id: u128 = rand::random();

    let (input_ty, input_pat) = match inputs.len() {
        0 => (quote! { () }, quote! {_}),
        1 => {
            let input = inputs.first().unwrap();
            let input_ty = &input.ty;
            let input_pat = &input.pat;
            (quote! { #input_ty }, quote! { #input_pat })
        }
        _ => {
            let types: Vec<_> = inputs.iter().map(|x| x.ty.clone()).collect();
            let patterns: Vec<_> = inputs.iter().map(|x| x.pat.clone()).collect();

            let input_ty = quote! {
                (#(#types),*)
            };
            let input_pat = quote! {
                (#(#patterns),*)
            };
            (input_ty, input_pat)
        }
    };

    let f_block = f.block;
    let f_block = if f.sig.asyncness.is_some() {
        quote! {
            {
                async_std::task::block_on(async #f_block)
            }
        }
    } else {
        quote! {
            #f_block
        }
    };

    TokenStream::from(quote! {
        #[allow(non_camel_case_types)]
        #f_vis struct #f_ident ;

        impl netsim_embed::MachineFn for #f_ident {
            type Arg = #input_ty ;

            fn id() -> u128 {
                #id
            }

            fn call( #input_pat : #input_ty) #f_block
        }
    })
}

#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macros/failures/*.rs");
    t.pass("tests/macros/success/*.rs");
}
