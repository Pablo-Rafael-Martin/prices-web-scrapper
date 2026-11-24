mod html_parser;
mod http_client;

use http_client::HttpClient;
use scraper::Html;

use crate::html_parser::parse_html;

#[tokio::main]
async fn main() {
    // instanciar o HttpClient
    println!("Instanciando cliente http...");
    let client = match HttpClient::new() {
        Ok(client) => {
            println!("Cliente http instanciado com sucesso.");
            client
        }
        Err(error) => {
            eprintln!("Falha ao instanciar o reqwest client: {error}");
            return;
        }
    };

    
}
