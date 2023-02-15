// ! This is a actix microserver supporting uploading a file to translate from English to Italian using huggingface's rust-bert library.
// ! The server is running on localhost:8080
// ! The server supports the following routes:
// ! /from/[lang1]/to/[lang2]/[contents] that returns the formatted json
// ! / that turns a hello world
// !
// ! The server supports the following languages:
// ! English
// ! Spanish
// ! French
// ! Italian
// ! German
// ! Dutch
// ! Portuguese
// ! Russian
// ! ChineseMandarin
// ! Japanese

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

// A.  / that turns a hello world

// B. /from/[lang1]/to/[lang2]/[contents] that returns the formatted json

fn map_language(lang: String) -> Language {
    let lang = lang.to_lowercase();
    match lang.as_str() {
        "english" => Language::English,
        "french" => Language::French,
        "german" => Language::German,
        "spanish" => Language::Spanish,
        _ => Language::English,
    }
}


// TODO: convert the following code to actix web
fn main() -> anyhow::Result<()> {
    let from = "english";
    let to = "french";

    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![map_language(from.to_string())])
        .with_target_languages(vec![Language::Spanish, Language::French, Language::Italian])
        .create_model()?;
    let input_text = "This is a sentence to be translated";
    let output = model.translate(&[input_text], None, map_language(to.to_string()))?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}
