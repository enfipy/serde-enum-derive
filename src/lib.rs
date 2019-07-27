#![feature(proc_macro_diagnostic, proc_macro_span)]
#![feature(core_intrinsics, decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate quote;
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

fn impl_serde_enum_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl FromSql for #name {
            fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
                let value = types::int8_from_sql(raw)?;
                Self::from_i64(value).ok_or(Box::from("Failed to deserialize enum"))
            }

            fn accepts(ty: &Type) -> bool {
                <i64 as ToSql>::accepts(ty)
            }
        }

        impl ToSql for #name {
            fn to_sql(&self, _: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn Error + Sync + Send>>
            where
                Self: Sized,
            {
                let res = self.to_i64().ok_or(Box::from("Failed to serialize enum"))?;
                types::int8_to_sql(res, out);
                Ok(IsNull::No)
            }

            fn accepts(ty: &Type) -> bool {
                <i64 as ToSql>::accepts(ty)
            }

            to_sql_checked!();
        }
    }
}

#[proc_macro_derive(EnumSql)]
pub fn serde_enum_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_serde_enum_derive(&ast);
    gen.parse().unwrap()
}
