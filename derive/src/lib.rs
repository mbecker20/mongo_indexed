use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Field};

/// The macro requires you import `Document` type yourself eg `use bson::Document;`
#[proc_macro_derive(
    MongoIndexed,
    attributes(
        collection_name,
        doc_index,
        unique_doc_index,
        sparse_doc_index,
        index,
        unique_index,
        sparse_index,
    )
)]
pub fn derive_indexed(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input as DeriveInput);

    let mut doc_indexes = Vec::new();
    let mut unique_doc_indexes = Vec::new();
    let mut sparse_doc_indexes = Vec::new();
    let mut collection_name = ident.clone();

    for attr in attrs {
        if attr.path().is_ident("unique_doc_index") {
            let doc = attr
                .parse_args::<Expr>()
                .expect("unique_doc_index: expected doc! macro");
            unique_doc_indexes.push(doc);
        }
        if attr.path().is_ident("sparse_doc_index") {
            let doc = attr
                .parse_args::<Expr>()
                .expect("sparse_doc_index: expected doc! macro");
            sparse_doc_indexes.push(doc);
        }
        if attr.path().is_ident("doc_index") {
            let doc = attr
                .parse_args::<Expr>()
                .expect("doc_index: expected doc! macro");
            doc_indexes.push(doc);
        }
        if attr.path().is_ident("collection_name") {
            collection_name = attr.parse_args().expect("collection_name: should be ident");
        }
    }

    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("must derive on struct"),
    };

    let mut indexes = Vec::new();
    let mut unique_indexes = Vec::new();
    let mut sparse_indexes = Vec::new();

    for Field { attrs, ident, .. } in s.fields {
        let Some(ident) = ident else {
            continue;
        };
        let ident = quote!(stringify!(#ident));
        let is_unique = attrs
            .iter()
            .any(|attr| attr.path().is_ident("unique_index"));
        if is_unique {
            unique_indexes.push(ident);
            continue;
        }
        let is_sparse = attrs
            .iter()
            .any(|attr| attr.path().is_ident("sparse_index"));
        if is_sparse {
            sparse_indexes.push(ident);
            continue;
        }
        let is_index = attrs.iter().any(|attr| attr.path().is_ident("index"));
        if is_index {
            indexes.push(ident);
            continue;
        }
    }

    quote! {
        impl mongo_indexed::Indexed for #ident {
            fn default_collection_name() -> &'static str {
                stringify!(#collection_name)
            }
            fn indexes() -> &'static [&'static str] {
                &[#(#indexes,)*]
            }
            fn unique_indexes() -> &'static [&'static str] {
                &[#(#unique_indexes,)*]
            }
            fn sparse_indexes() -> &'static [&'static str] {
                &[#(#sparse_indexes,)*]
            }
            fn doc_indexes() -> Vec<Document> {
                vec![#(#doc_indexes,)*]
            }
            fn unique_doc_indexes() -> Vec<Document> {
                vec![#(#unique_doc_indexes,)*]
            }
            fn sparse_doc_indexes() -> Vec<Document> {
                vec![#(#sparse_doc_indexes,)*]
            }
        }
    }
    .into()
}
