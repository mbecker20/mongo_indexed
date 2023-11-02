use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Field};

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
        skip_index,
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
        Data::Enum(_) => {
            return quote! {
                impl mongo_indexed::Indexed for #ident {
                    fn doc_indexes() -> Vec<mongo_indexed::Document> {
                        vec![#(#doc_indexes,)*]
                    }
                    fn unique_doc_indexes() -> Vec<mongo_indexed::Document> {
                        vec![#(#unique_doc_indexes,)*]
                    }
                    fn sparse_doc_indexes() -> Vec<mongo_indexed::Document> {
                        vec![#(#sparse_doc_indexes,)*]
                    }
                }
            }
            .into()
        }
        _ => panic!("must derive on struct or enum"),
    };

    let mut indexes = Vec::new();
    let mut unique_indexes = Vec::new();
    let mut sparse_indexes = Vec::new();

    for Field {
        attrs, ident, ty, ..
    } in s.fields
    {
        if ident.is_none() {
            continue;
        }
        let ident = ident.unwrap();
        let skip = attrs.iter().any(|attr| attr.path().is_ident("skip_index"));
        if skip {
            continue;
        }
        let is_unique = attrs
            .iter()
            .any(|attr| attr.path().is_ident("unique_index"));
        if is_unique {
            unique_indexes.push(quote! {
                unique_indexes.push(stringify!(#ident).to_string());
            });
            continue;
        }
        let is_sparse = attrs
            .iter()
            .any(|attr| attr.path().is_ident("sparse_index"));
        if is_sparse {
            sparse_indexes.push(quote! {
                sparse_indexes.push(stringify!(#ident).to_string());
            });
            continue;
        }
        let is_index = attrs.iter().any(|attr| attr.path().is_ident("index"));
        if is_index {
            indexes.push(quote! {
                indexes.push(stringify!(#ident).to_string());
            });
            continue;
        }
        indexes.push(quote! {
            let nested = <#ty as mongo_indexed::Indexed>::indexes();
            for nested in nested {
                indexes.push(format!("{}.{}", stringify!(#ident), nested));
            }
        });
        unique_indexes.push(quote! {
            let nested = <#ty as mongo_indexed::Indexed>::unique_indexes();
            for nested in nested {
                unique_indexes.push(format!("{}.{}", stringify!(#ident), nested));
            }
        });
        sparse_indexes.push(quote! {
            let nested = <#ty as mongo_indexed::Indexed>::sparse_indexes();
            for nested in nested {
                sparse_indexes.push(format!("{}.{}", stringify!(#ident), nested));
            }
        });
    }

    quote! {
        impl mongo_indexed::Indexed for #ident {
            fn default_collection_name() -> &'static str {
                stringify!(#collection_name)
            }
            fn indexes() -> Vec<String> {
                let mut indexes = Vec::new();
                #(#indexes)*
                indexes
            }
            fn unique_indexes() -> Vec<String> {
                let mut unique_indexes = Vec::new();
                #(#unique_indexes)*
                unique_indexes
            }
            fn sparse_indexes() -> Vec<String> {
                let mut sparse_indexes = Vec::new();
                #(#sparse_indexes)*
                sparse_indexes
            }
            fn doc_indexes() -> Vec<mongo_indexed::Document> {
                vec![#(#doc_indexes,)*]
            }
            fn unique_doc_indexes() -> Vec<mongo_indexed::Document> {
                vec![#(#unique_doc_indexes,)*]
            }
            fn sparse_doc_indexes() -> Vec<mongo_indexed::Document> {
                vec![#(#sparse_doc_indexes,)*]
            }
        }
    }
    .into()
}
