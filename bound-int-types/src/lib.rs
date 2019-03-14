extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, LitInt, Token,
};

struct BoundedInt {
    name: Ident,
    lower: LitInt,
    upper: LitInt,
}

impl Parse for BoundedInt {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token!(,)>()?;
        let lower: LitInt = input.parse()?;
        input.parse::<Token!(,)>()?;
        let upper: LitInt = input.parse()?;

        Ok(BoundedInt { name, lower, upper })
    }
}

#[proc_macro]
pub fn bound_int_types(input: TokenStream) -> TokenStream {
    let BoundedInt { name, lower, upper } = syn::parse_macro_input!(input);

    let lower = lower.value();
    let upper = upper.value();

    // Define the trait, which is the general type for the bounded integer.
    let mut out = quote! {
        trait #name: std::fmt::Debug {
            fn get() -> Self;
            fn value(&self) -> u64;
        }
    };

    for i in lower..=upper {
        let name_i = format!("{}_{}", name, i);
        let struct_name = format!("__{}_struct", name_i);
        let addable_name = format!("__{}_addable", name_i);

        let name_i_ident = Ident::new(&name_i, Span::call_site());
        let struct_name_ident = Ident::new(&struct_name, Span::call_site());
        let addable_name_ident = Ident::new(&addable_name, Span::call_site());

        out.extend(quote! {
            // Define the type for the specific value.
            #[derive(Copy, Clone, Debug, PartialEq)]
            struct #struct_name_ident {}

            impl #struct_name_ident {
                fn get_sum<A: #addable_name_ident>(&self, other: A) -> A::Result {
                    other.sum()
                }
            }

            // Define a constant for external usage of the value.
            const #name_i_ident: #struct_name_ident = #struct_name_ident {};

            // Implement the general trait on the struct.
            impl #name for #struct_name_ident {
                fn get() -> Self {
                    #name_i_ident
                }

                fn value(&self) -> u64 {
                    #i
                }
            }

            // Define a trait for values which can be added to this value.
            trait #addable_name_ident: #name {
                type Result: #name;

                fn sum(&self) -> Self::Result {
                    Self::Result::get()
                }
            }
        });

        // Implement the "addable" traits for the value which won't overflow.
        //
        // For example, if `lower` is 1, `upper` is `10`, and `i` is `6`, then the numbers that can
        // be added to `6` are `1` through `4`.
        for addable in lower..=upper - i {
            let addable_impl = format!("__{}_{}_addable", name, addable);
            let result = format!("__{}_{}_struct", name, addable + i);

            let addable_impl_ident = Ident::new(&addable_impl, Span::call_site());
            let result_ident = Ident::new(&result, Span::call_site());

            out.extend(quote!{
                impl #addable_impl_ident for #struct_name_ident {
                    type Result = #result_ident;
                }
            });
        }
    }

    TokenStream::from(out)
}
