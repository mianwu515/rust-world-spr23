use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};

fn main() -> anyhow::Result<()> {
let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish, Language::French, Language::Italian])
        .create_model()?;
    let input_text = "This is a sentence to be translated";
    let output = model.translate(&[input_text], None, Language::French)?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}