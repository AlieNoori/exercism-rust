use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Planet)]
pub fn derive_planet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let period = match name.to_string().as_str() {
        "Mercury" => 0.2408467,
        "Venus" => 0.61519726,
        "Earth" => 1.0,
        "Mars" => 1.8808158,
        "Jupiter" => 11.862615,
        "Saturn" => 29.457615,
        "Uranus" => 84.016846,
        "Neptune" => 164.79132,
        _ => panic!("Unknown planet"),
    };

    quote! {
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                d.secs as f64 / (31557600.0 * #period)
            }
        }
    }
    .into()
}
