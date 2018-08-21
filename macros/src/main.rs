#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec.push(1);
            temp_vec
        }
    };
}


// extern crate proc_macro;
// extern crate syn;
// #[macro_use]
// extern crate quote;

// use proc_macro::TokenStream;

// trait HelloMacro {
//     fn hello_macro();
// }

// #[proc_macro_derive(HelloMacro)]
// fn hello_macro_derive(input: TokenStream) -> TokenStream {
//     // Construct a string representation of the type definition
//     let s = input.to_string();

//     // Parse the string representation
//     let ast = syn::parse_derive_input(&s).unwrap();

//     // Build the impl
//     let gen = impl_hello_macro(&ast);

//     // Return the generated impl
//     gen.parse().unwrap()
// }

// fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
//     let name = &ast.ident;
//     quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}", stringify!(#name));
//             }
//         }
//     }
// }


// #[derive(HelloMacro)]
// struct Pancakes;

fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", v);

    // Pancakes::hello_macro();
}
