use proc_macro2::TokenStream;
use syn::{Expr, Ident, ItemFn, ItemStruct, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Iter, Punctuated};
use quote::{quote, ToTokens};


pub enum OptArgsItem {
    Function(ItemFn),
    Struct(ItemStruct),
}


impl Parse for OptArgsItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let function: syn::Result<ItemFn> = input.parse();
        match function {
            Ok(function) => Ok(Self::Function(function)),
            _ => Ok(Self::Struct(input.parse()?))
        }
    }
}


impl ToTokens for OptArgsItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OptArgsItem::Function(f) => f.to_tokens(tokens),
            OptArgsItem::Struct(s) => s.to_tokens(tokens)
        }
    }
}


pub struct OptArgs {
    pub attrs: Punctuated<DefaultValue, Token![,]>,
}


impl Parse for OptArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(OptArgs { attrs: input.parse_terminated(DefaultValue::parse)? })
    }
}


impl OptArgs {
    pub fn iter(&self) -> Iter<DefaultValue> {
        self.attrs.iter()
    }
}


pub struct DefaultValue {
    pub name: String,
    pub value: String,
}


impl Parse for DefaultValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        if input.lookahead1().peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;
            Ok(DefaultValue {
                name: quote!(#name).to_string(),
                value: quote!(#value).to_string(),
            })
        } else {
            Ok(DefaultValue {
                name: quote!(#name).to_string(),
                value: String::from("core::default::Default::default()"),
            })
        }
    }
}


impl ToString for DefaultValue {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}
