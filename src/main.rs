use reqwest::Client;
use std::env;
use dotenv::dotenv;

#[derive(Debug)]
struct PokemonProduct {
    url: String,
    name: String,
}

async fn send_telegram_message(token: &str, chat_id: &str, message: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = Client::new();

    let params = [
        ("chat_id", chat_id),
        ("text", message),
    ];

    client.post(&url)
        .form(&params)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");
    let chat_id = env::var("CHAT_ID").expect("CHAT_ID environment variable not set");

    let mut pokemon_products: Vec<PokemonProduct> = Vec::new(); 
    let browser = headless_chrome::Browser::default().unwrap(); 

    let tab = browser.new_tab().unwrap(); 
    tab.navigate_to("https://uk.jooble.org/SearchResult?rgns=&ukw=backend%20developer").unwrap();  
    tab.set_default_timeout(std::time::Duration::from_secs(50));

    let job_cards = tab.wait_for_elements("div.ojoFrF").unwrap();

    for job_card in job_cards {
        let name = job_card
            .wait_for_element("h2.sXM9Eq")
            .unwrap()
            .get_inner_text()
            .unwrap();

        let url = job_card
            .wait_for_element("a.tUC4Fj")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap() 
            .get(7)
            .unwrap()
            .to_owned();  

        println!("{} - {}", name, url);

        let pokemon_product = PokemonProduct {
            url,
            name,
        };

        pokemon_products.push(pokemon_product);
    }

      let message = pokemon_products
        .iter()
        .map(|product| format!("{} - {}", product.name, product.url))
        .collect::<Vec<String>>()
        .join("\n");

      match send_telegram_message(&token, &chat_id, &message).await {
         Ok(_) => println!("Message sent successfully!"),
         Err(e) => println!("Failed to send message: {:?}", e),
       }
}
