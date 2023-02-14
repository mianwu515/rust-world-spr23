use rust_bert::pipelines::sentiment::SentimentModel;
use anyhow::{Context, Result};


fn main() {
    let input = [
        "Probably my all-time favorite movie, a story of selflessness, sacrifice and dedication to a noble cause, but it's not preachy or boring.",
        "This film tried to be too many things all at once: stinging political satire, Hollywood blockbuster, sappy romantic comedy, family values promo...",
        "If you like original gut wrenching laughter you will like this movie. If you are young or old then you will love this movie, hell even my mom liked it.",
    ];
    let sentiment_model = SentimentModel::new(Default::default()).unwrap();
    for sentence in input.iter() {
        let output = sentiment_model.predict(sentence as &AsRef<str>, None);
        println!("{:?}", output);
    }
}
