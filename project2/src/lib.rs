// #[get("/from/{lang1}/to/{lang2}/{contents}")]
async fn translate_from_to(
    lang1: web::Path<String>,
    lang2: web::Path<String>,
    contents: web::Path<String>,
) -> impl Responder {
    let from: String = lang1.to_string().to_lowercase();
    let to: String = lang2.to_string().to_lowercase();
    // a set of supported languages
    let supported_languages = vec![
        "english",
        "Spanish",
        "French",
        "Italian",
        "German",
        "Dutch",
        "Portuguese",
        "Russian",
        "ChineseMandarin",
        "Japanese",
        "Korean",
    ];
    // iterate through the supported languages and check if the input language is supported
    let mut from_supported = false;
    let mut to_supported = false;
    for language in supported_languages {
        if language == from {
            from_supported = true;
        }
        if language == to {
            to_supported = true;
        }
    }
    // if the input language is not supported, return an error
    if !from_supported {
        return HttpResponse::BadRequest().body(format!("{} is not a supported language", from));
    }
    if !to_supported {
        return HttpResponse::BadRequest().body(format!("{} is not a supported language", to));
    }

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