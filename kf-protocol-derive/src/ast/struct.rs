use crate::ast::prop::Prop;
use syn::{Fields, Generics, Ident, ItemStruct};

#[derive(Debug)]
pub(crate) struct KfStruct {
    pub struct_ident: Ident,
    pub props: Vec<Prop>,
    pub generics: Generics,
}

impl KfStruct {
    pub fn from_ast(item: &ItemStruct) -> syn::Result<Self> {
        let struct_ident = item.ident.clone();
        let props: Vec<Prop> = if let Fields::Named(fields) = &item.fields {
            fields
                .named
                .iter()
                .map(|field| Prop::from_ast(field).expect("Could not parse field level attribute."))
                .collect()
        } else {
            vec![]
        };
        let generics = item.generics.clone();

        Ok(KfStruct {
            struct_ident,
            props,
            generics,
        })
    }
}
