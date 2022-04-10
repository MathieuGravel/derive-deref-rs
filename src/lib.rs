#[cfg(test)]
mod tests;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Field, Fields, Generics, Type};

#[proc_macro_derive(Deref, attributes(deref))]
pub fn derive_deref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_deref_inner(input.into())
        .unwrap_or_else(|e| syn::Error::to_compile_error(&e))
        .into()
}

struct DeriveDerefData {
    field: TokenStream,
    ty: Type,
}

fn derive_deref_inner(input: TokenStream) -> syn::Result<TokenStream> {
    let DeriveInput {
        data,
        generics,
        ident,
        ..
    } = syn::parse2(input)?;

    let deref_field = get_deref_field(get_struct_fields(data)?)?;

    Ok(deref_impl_token_stream(ident, generics, deref_field))
}

fn get_struct_fields(data: Data) -> syn::Result<Fields> {
    match data {
        Data::Struct(struct_data) => Ok(struct_data.fields),
        Data::Enum(enum_data) => Err(syn::Error::new_spanned(
            enum_data.enum_token,
            "Derive Deref does not support Enum.",
        )),
        Data::Union(union_data) => Err(syn::Error::new_spanned(
            union_data.union_token,
            "Derive Deref does not support Union.",
        )),
    }
}

fn get_deref_field(fields: Fields) -> syn::Result<DeriveDerefData> {
    let fields_span = fields.span();
    let nb_fields = fields.len();
    let mut field_iter = fields.into_iter();

    let derive_deref_data = if nb_fields > 1 {
        let mut derive_field_iter = field_iter.filter(|f| check_for_derive_helper_attribute(f));
        let field = derive_field_iter.nth(0).ok_or(syn::Error::new(
            fields_span,
            "Struct with multiple field need to have the #[deref] attribute on one field.",
        ))?;
        let field_span = Span::clone(&field.span());
        if let Some(next_field) = derive_field_iter.next() {
            Err(syn::Error::new_spanned(
                next_field,
                "Only one field can have the attribute #[deref]",
            ))?;
        }
        let field_ident = field.ident.ok_or(syn::Error::new(
            field_span,
            "Struct field need to be identified when there are multiple.",
        ))?;
        DeriveDerefData {
            field: field_ident.into_token_stream(),
            ty: field.ty,
        }
    } else {
        if let Some(field) = field_iter.nth(0) {
            let field_ident = field
                .ident
                .map(|i| i.to_token_stream())
                .unwrap_or(quote! { 0 });
            DeriveDerefData {
                field: field_ident,
                ty: field.ty,
            }
        } else {
            Err(syn::Error::new(
                fields_span,
                "Derive Deref cannot deref empty struct.",
            ))?
        }
    };
    Ok(derive_deref_data)
}

fn check_for_derive_helper_attribute(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.to_token_stream().to_string() == "deref")
        .count()
        >= 1
}

fn deref_impl_token_stream(
    struct_ident: Ident,
    generics: Generics,
    DeriveDerefData { field, ty }: DeriveDerefData,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {

        impl #impl_generics core::ops::Deref for #struct_ident #ty_generics #where_clause {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self.#field
            }
        }

        impl #impl_generics core::ops::DerefMut for #struct_ident #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#field
            }
        }
    }
}
