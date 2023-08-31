use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CharCard {
    name: String,
    char: String,
    knowledge: u16,
    definitions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct PhraseCard {
    name: String,
    char: Vec<CharCard>,
    knowledge: u16,
    definitions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
enum Card {
    Char(CharCard),
    Phrase(PhraseCard),
}

#[derive(Serialize, Deserialize)]
struct Deck { 
    deck: Vec<Card>,
    knowldege: u16 
}

#[derive(Serialize, Deserialize)]
struct YouyinUser {
    is_premium: bool,
    decks: Vec<Deck>,
    combo: u16,
    streak: u16,
    sessions: u64,
    last_session: u64,
}

async fn handle(_req: tide::Request<()>) -> tide::Result<String> {
    Ok("Hello, World!".to_string()) 
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(handle);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
