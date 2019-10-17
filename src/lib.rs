extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn;

/// Macro to test if host system has a command
///
/// ```rust
/// use has_command::has_command;
///
/// use std::process::Command;
///
/// fn main() {
///     run_ls();
///     run_not_a_command();
///     run_multiple();
/// }
///
/// #[has_command(ls)]
/// fn run_ls() {
///     Command::new("ls").output().expect("Error running ls");
/// }
///
/// #[has_command(not-a-command)]
/// fn run_not_a_command() {
///     assert!(false);
/// }
///
/// #[has_command(ls)]
/// #[has_command(ps)]
/// #[has_command(cat)]
/// fn run_multiple() {
///     assert!(Command::new("ls").output().expect("Error running ls").status.success());
///     assert!(Command::new("ps").output().expect("Error running ps").status.success());
///     assert!(Command::new("not-a-command").output().is_err());
///     assert!(!Command::new("cat").arg("not-a-file").output().expect("Error running cat").status.success());
/// }
/// ```
#[proc_macro_attribute]
pub fn has_command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attrs = attr.into_iter().collect::<Vec<TokenTree>>();
    let mut command = String::new();
    for tk in attrs {
        match tk {
            TokenTree::Ident(id) => {
                command += &id.to_string();
            },
            TokenTree::Punct(a) if a.to_string() != "," => {
                command += &a.to_string();
            },
            o => {
                panic!("Not supported token! {:?}", o.to_string());
            }
        };
    }

    let ast: syn::ItemFn = syn::parse(input).unwrap();

    let block = ast.block;
    let signature = ast.sig;
    let attrs = ast.attrs;
    let gen = quote! {
        #(#attrs)
        *
        #signature {
            use std::process::Command;

            match Command::new(#command).output() {
                Err(_) => {},
                Ok(_) => {
                    #block
                }
            }
        }
    };

    return gen.into();
}
