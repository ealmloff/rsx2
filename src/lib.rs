use kalosm::language::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::LitStr;
use tokio::runtime::Runtime;

use dioxus_rsx as rsx;

/// The rsx! macro makes it easy for developers to write jsx-style markup in their components.
#[proc_macro]
pub fn rsx2(tokens: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(tokens) {
        Err(err) => err.to_compile_error().into(),
        Ok(body) => generate_rsx(body),
    }
}

fn try_generate_rsx(string: String) -> String {
    Runtime::new().unwrap().block_on(async move {
        let llm = Llama::new_chat().await.unwrap();

        let constraints =
            RegexParser::new(r#"[\w_\{\}" ]{5,200}"#).unwrap();

        let task = Task::builder("You are an assistant who converts natural language to rsx. Rsx is similar to HTML except it uses braces instead of starting and closing tags. It also replaces any -s with _s.")
            .with_constraints(constraints)
            .with_example("Hello world", r#""hello world""#)
            .with_example("5 buttons", "button {}\nbutton {}\nbutton {}\nbutton {}\nbutton {}")
            .with_example("5 buttons with text", "button { text: \"button 1\" }\nbutton { text: \"button 2\" }\nbutton { text: \"button 3\" }\nbutton { text: \"button 4\" }\nbutton { text: \"button 5\" }")
            .with_example("Give me buttons!!!", "button {}\nbutton {}\nbutton {}\nbutton {}\nbutton {}")
            .build();

        loop {
            let rsx_string = task.run(&string, &llm).text().await;
            if syn::parse_str::<rsx::CallBody>(&rsx_string).is_ok() {
                break rsx_string;
            }
        }
    })
}

fn generate_rsx(body: LitStr) -> TokenStream {
    let string = body.value();

    let rsx_string = try_generate_rsx(string.clone());
    syn::parse_str::<rsx::CallBody>(&rsx_string)
        .unwrap()
        .into_token_stream()
        .into()
}
