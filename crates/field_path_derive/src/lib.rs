//! # Steps
//!
//! 1. `#[remember_path]` attribute on `use xx::yy::{zz, zzz};` to remember the full path (of `zz` & `zzz` in this case).
//! 2. `#[remember_fields]` attribute on `struct X { field: Ty }` to remember the struct's token (with full path).
//! 3. `#[register_fields]` attribute on `struct X { field: Ty }` to generated field paths and accessors for each field
//!    in the `struct`.

use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::{TokenStreamExt, quote};
use syn::spanned::Spanned;
use syn::{
    Error, Ident, ItemStruct, ItemUse, UseTree, parse,
    parse_macro_input,
};

macro_rules! type_stringify_array {
    ($($t:ty),+ $(,)?) => {
        &[
            $(
                {
                    // Making sure that the type is real.
                    const _: Option<&$t> = None;
                    stringify!($t)
                }
            ),+
        ]
    }
}

const FULL_PATH_MACRO_PREFIX: &str = "___full_path_";
const PRIMITIVE_TYPES: &[&str] = type_stringify_array!(
    (),
    // Integers.
    usize,
    isize,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    // Floats.
    f32,
    f64,
    bool,
    // Strings.
    char,
    str,
);

#[proc_macro_attribute]
pub fn remember_fields(
    _attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream {
    let item = parse_macro_input!(tokens as ItemStruct);

    match remember_fields_impl(item) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

fn remember_fields_impl(
    item: ItemStruct,
) -> Result<TokenStream, Error> {
    Ok(quote! {
        #item
    }
    .into())
}

#[proc_macro_attribute]
pub fn remember_path(
    _attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream {
    match remember_path_impl(tokens) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

fn remember_path_impl(
    tokens: TokenStream,
) -> Result<TokenStream, Error> {
    let item = parse::<ItemUse>(tokens)?;

    if let UseTree::Name(use_name) = &item.tree {
        return Err(Error::new(
            use_name.span(),
            "Remembering the path of a single name is useless.",
        ));
    }

    let mut trees = vec![&item.tree];

    let mut path_chain: Vec<Ident> = Vec::new();
    let mut names: Vec<(usize, Ident, Option<Ident>)> = Vec::new();

    while let Some(tree) = trees.pop() {
        let path_idx = path_chain.len();
        match tree {
            UseTree::Path(use_path) => {
                path_chain.push(use_path.ident.clone());
                trees.push(&use_path.tree);
            }
            UseTree::Name(use_name) => {
                names.push((path_idx, use_name.ident.clone(), None));
            }
            UseTree::Rename(use_rename) => names.push((
                path_idx,
                use_rename.ident.clone(),
                Some(use_rename.rename.clone()),
            )),
            UseTree::Glob(use_glob) => {
                return Err(Error::new(
                    use_glob.span(),
                    "Wildcard cannot be used with this attribute!",
                ));
            }
            UseTree::Group(use_group) => {
                trees.extend(use_group.items.iter());
            }
        }
    }

    let has_leading_colon = item.leading_colon.is_some();

    let macros = names
        .into_iter()
        .map(|(path_idx, name, rename)| {
            let macro_ident = Ident::new(
                &format!(
                    "{FULL_PATH_MACRO_PREFIX}{}",
                    rename.unwrap_or_else(|| name.clone())
                ),
                Span2::call_site(),
            );

            let mut path = TokenStream2::new();
            if has_leading_colon {
                path.append_all(quote!(::));
            }
            for p in path_chain.iter().take(path_idx) {
                path.append_all(quote!(#p::));
            }
            path.append(name);

            quote! {
                #[doc(hidden)]
                macro_rules! #macro_ident {
                    () => {
                        #path
                    }
                }
            }
        })
        .fold(TokenStream2::new(), |a, b| {
            quote! {
                #a
                #b
            }
        });

    Ok(quote! {
        #item
        #macros
    }
    .into())
}

#[proc_macro_attribute]
pub fn print_tokens(
    _attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream {
    println!("{tokens}");

    tokens
}
