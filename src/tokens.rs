use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::parser::{
    OptArgsItem, OptArgsItemFn, OptArgsItemFnArg, OptArgsItemStruct, OptArgsItemStructFields,
    OptArgsItemType,
};

impl ToTokens for OptArgsItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let OptArgsItem {
            attrs,
            vis,
            constness,
            asyncness,
            unsafety,
            abi,
            item,
        } = self;
        match item {
            OptArgsItemType::ItemFn(OptArgsItemFn {
                ident,
                generics,
                inputs,
                output,
                block,
                ..
            }) => quote!(
                #(#attrs)*
                #vis #constness #asyncness #unsafety #abi fn #ident
                #generics
                (#(#inputs),*) #output #block
            )
            .to_tokens(tokens),
            OptArgsItemType::ItemStruct(OptArgsItemStruct {
                ident,
                generics,
                fields,
                semi_token,
                ..
            }) => quote!(
                #(#attrs)*
                #vis
                struct #ident #generics {
                    #(#fields),*
                } #semi_token
            )
            .to_tokens(tokens),
        }
    }
}

impl ToTokens for OptArgsItemFnArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let OptArgsItemFnArg {
            attrs, ident, ty, ..
        } = self;
        quote!(
            #(#attrs)*
            #ident: #ty
        )
        .to_tokens(tokens)
    }
}

impl ToTokens for OptArgsItemStructFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let OptArgsItemStructFields {
            attrs,
            vis,
            mutability,
            ident,
            ty,
            ..
        } = self;
        quote!(
            #(#attrs)*
            #vis #mutability #ident: #ty
        )
        .to_tokens(tokens)
    }
}
