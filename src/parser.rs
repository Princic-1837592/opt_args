use derive_syn_parse::Parse;
use proc_macro2::{Ident, Span};
use syn::{
    parse::{Parse, ParseStream},
    token::{Async, Brace, Colon, Const, Paren, Semi, Struct, Unsafe},
    Abi, Attribute, Block, Error, Expr, Generics, ReturnType, Token, Type, Visibility,
};

#[derive(Parse, Clone)]
pub(crate) struct OptArgsItem {
    #[call(Attribute::parse_outer)]
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub constness: Option<Const>,
    pub asyncness: Option<Async>,
    pub unsafety: Option<Unsafe>,
    pub abi: Option<Abi>,
    pub item: OptArgsItemType,
}

#[derive(Clone)]
pub(crate) enum OptArgsItemType {
    ItemFn(OptArgsItemFn),
    ItemStruct(OptArgsItemStruct),
}

impl OptArgsItemType {
    pub fn ident(&self) -> &Ident {
        match self {
            OptArgsItemType::ItemFn(item_fn) => &item_fn.ident,
            OptArgsItemType::ItemStruct(item_struct) => &item_struct.ident,
        }
    }
}

impl Parse for OptArgsItemType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![fn]) {
            Ok(Self::ItemFn(input.parse()?))
        } else if input.peek(Token![struct]) {
            Ok(Self::ItemStruct(input.parse()?))
        } else {
            Err(Error::new(
                Span::call_site(),
                "`opt_args` can only be applied to functions or structs",
            ))
        }
    }
}

#[derive(Parse, Clone)]
pub(crate) struct OptArgsItemFn {
    _fn_token: Token![fn],
    pub ident: Ident,
    pub generics: Generics,
    #[paren]
    _paren_token: Paren,
    #[inside(_paren_token)]
    #[call(parse_vector)]
    pub inputs: Vec<OptArgsItemFnArg>,
    pub output: ReturnType,
    pub block: Box<Block>,
}

#[derive(Parse, Clone)]
pub(crate) struct OptArgsItemFnArg {
    #[call(Attribute::parse_outer)]
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    _colon_token: Colon,
    pub ty: Box<Type>,
    _eq: Option<Token![=]>,
    #[parse_if(_eq.is_some())]
    pub value: Option<Expr>,
    #[parse_if(_eq.is_none())]
    pub default: Option<Option<Token![?]>>,
}

#[derive(Parse, Clone)]
pub(crate) struct OptArgsItemStruct {
    _struct_token: Struct,
    pub ident: Ident,
    pub generics: Generics,
    #[brace]
    _paren_token: Brace,
    #[inside(_paren_token)]
    #[call(parse_vector)]
    pub fields: Vec<OptArgsItemStructFields>,
    pub semi_token: Option<Semi>,
}

#[derive(Parse, Clone)]
pub(crate) struct OptArgsItemStructFields {
    #[call(Attribute::parse_outer)]
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    _colon_token: Colon,
    pub ty: Type,
    _eq: Option<Token![=]>,
    #[parse_if(_eq.is_some())]
    pub value: Option<Expr>,
    #[parse_if(_eq.is_none())]
    pub default: Option<Option<Token![?]>>,
}

fn parse_vector<T: Parse>(input: ParseStream) -> syn::Result<Vec<T>> {
    Ok(input
        .parse_terminated(T::parse, Token![,])?
        .into_iter()
        .collect())
}

#[derive(Clone)]
pub(crate) struct GenericOptArg {
    pub ident: Ident,
    pub ty: Box<Type>,
    pub value: Option<Expr>,
    pub default: bool,
}

impl GenericOptArg {
    pub fn is_optional(&self) -> bool {
        self.value.is_some() || self.default
    }
}

impl From<OptArgsItemFnArg> for GenericOptArg {
    fn from(arg: OptArgsItemFnArg) -> Self {
        Self {
            ident: arg.ident,
            ty: arg.ty,
            value: arg.value,
            default: matches!(arg.default, Some(Some(_))),
        }
    }
}

impl From<OptArgsItemStructFields> for GenericOptArg {
    fn from(arg: OptArgsItemStructFields) -> Self {
        Self {
            ident: arg.ident,
            ty: Box::new(arg.ty),
            value: arg.value,
            default: matches!(arg.default, Some(Some(_))),
        }
    }
}
