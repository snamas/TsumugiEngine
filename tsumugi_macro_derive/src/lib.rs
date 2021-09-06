use proc_macro::{TokenStream, TokenTree, Span};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};
use std::str::FromStr;

#[proc_macro_derive(TsumugiAny)]
pub fn derive_any(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl ::tsumugi_macro::TsumugiAnyTrait for #name{
            fn as_any(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
    gen.into()
}
#[proc_macro]
pub fn pack(input: TokenStream) -> TokenStream {
    dbg!(&input);
    let mut tokenvec: Vec<TokenStream> = vec![];
    let mut parcevec: TokenStream = TokenStream::new();

    for token in input {
        dbg!(&token.span());
        match token {
            TokenTree::Group(_) => {
                parcevec.extend(Some(token.clone()));
            }
            TokenTree::Ident(_) => {
                parcevec.extend(Some(token.clone()));
            }
            TokenTree::Punct(_) => {
                if parcevec.is_empty() {} else {
                    tokenvec.push(parcevec.clone());
                }
                parcevec = TokenStream::new();
            }
            TokenTree::Literal(x) => {
                return err(x.span(), "構造体のみ入れることができます");
            }
        }
    }
    if parcevec.is_empty() {} else {
        tokenvec.push(parcevec.clone());
    }
    dbg!(&tokenvec);
    let mut quotedtoken: String = String::from("{");
    let mut antenna_chain_str = String::new();
    let mut tuple_antenna_chain_str = String::new();
    for (i, val) in tokenvec.iter().enumerate() {
        quotedtoken.push_str(
            &format!("let (mut antenna{0},mut tuple{0}) = {1}.create_tsumugi_antenna();", i,val.to_string()));
        antenna_chain_str.push_str(&format!("antenna{0},", i));
        tuple_antenna_chain_str.push_str(&format!("tuple{0},", i))
    }
    quotedtoken.push_str(
        &format!("TsumugiAntennaChain::from(({0}),vec![{1}])", tuple_antenna_chain_str, antenna_chain_str));
    quotedtoken.push_str("}");
    FromStr::from_str(&quotedtoken).unwrap()

    // FromStr::from_str("1").unwrap();
}

fn err(span: Span, msg: impl Into<String>) -> TokenStream {
    let msg = msg.into();
    quote_spanned!(span.into()=> {
        compile_error!(#msg);
    }).into()
}