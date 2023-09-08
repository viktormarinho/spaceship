use proc_macro2::{Group, Ident, Literal, Punct, Spacing, Span, TokenTree, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(err))]
struct EnumVariantAttrs {
    #[deluxe(default = 500)]
    pub status: usize,
    #[deluxe(default = String::from("Internal server error"))]
    pub message: String,
}

struct EnumVariantData {
    pub status: usize,
    pub message: String,
    pub identifier: String,
}

impl ToTokens for EnumVariantData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::new(self.identifier.as_str(), Span::call_site()));
        tokens.append(Punct::new('=', Spacing::Joint));
        tokens.append(Punct::new('>', Spacing::Alone));

        let statuscode_function_call_group = Group::new(
            proc_macro2::Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::usize_unsuffixed(self.status))]
                .into_iter()
                .collect(),
        );

        let stream: proc_macro2::TokenStream = [
            TokenTree::Ident(Ident::new("spaceship", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("axum", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("http", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("StatusCode", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("from_u16", Span::call_site())),
            TokenTree::Group(statuscode_function_call_group),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(proc_macro2::Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Literal(Literal::string(&self.message)),
        ]
        .into_iter()
        .collect();

        tokens.append(Group::new(proc_macro2::Delimiter::Parenthesis, stream));
    }
}

fn extract_variants_data(ast: &mut DeriveInput) -> deluxe::Result<Vec<EnumVariantData>> {
    let mut field_attrs: Vec<EnumVariantData> = Vec::new();

    if let syn::Data::Enum(e) = &mut ast.data {
        for field in e.variants.iter_mut() {
            let attrs: EnumVariantAttrs = deluxe::extract_attributes(field)?;
            field_attrs.push(EnumVariantData {
                status: attrs.status,
                message: attrs.message,
                identifier: field.ident.to_string(),
            });
        }
    }

    Ok(field_attrs)
}

fn err_response_derive_macro2(
    stream: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(stream)?;

    let variants: Vec<EnumVariantData> = extract_variants_data(&mut ast)?;

    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    Ok(quote::quote! {
        impl #impl_generics spaceship::axum::response::IntoResponse for #ident #type_generics #where_clause {
            fn into_response(self) -> spaceship::axum::response::Response {
                let (status, error_message) = match self {
                    #(#variants),*
                };
                let body = spaceship::axum::Json(spaceship::errors::ErrorResponseBody {
                    error: error_message.to_string()
                });
                (status, body).into_response()
            }
        }
    })
}

#[proc_macro_derive(ErrorResponse, attributes(err))]
pub fn err_response_derive_macro(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    err_response_derive_macro2(stream.into()).unwrap().into()
}
