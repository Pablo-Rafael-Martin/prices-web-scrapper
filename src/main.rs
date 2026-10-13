

mod http_client;
mod html_parser;
use html_parser::parse_html;
use std::result;

use http_client::HttpClient;

#[tokio::main]
async fn main() {
    // instanciar o HttpClient
    println!("Instanciando cliente http...");
    let client = match HttpClient::new() {
        Ok(client) => {
            println!("Cliente http instanciado com sucesso.");
            client
        },
        Err(error) => {
            eprintln!("Falha ao instanciar o reqwest client: {error}");
            return
        }
    };

    // chamar a url da pichau
    let pichau_url = "https://www.pichau.com.br/search?q=rx 9070 xt";
    
    println!("Buscando na Pichau...");
    let pichau_response = match client.new_request(pichau_url).await {
        Ok(response) => {
            println!("Busca na Pichau concluída.");
            response
        },
        Err(error) => {
            // ! SE DÁ ERRO ELE RETORNA, TEM QUE PENSAR EM COMO TRATAR ERRO SEM ENCERRAR A EXECUÇÃO
            
            eprintln!("Erro ao fazer requisição para a url '{pichau_url}':\n{error}");
            return
        }
    };

    // dar log no status
    let response_status_code = pichau_response.status();
    println!("Status code: {response_status_code}");

    // converter a resposta pra texto e parsear o html
    let response_html = match pichau_response.text().await {
        Ok(result) => {
            println!("Html convertido para texto com sucesso.");
            result
        },
        Err(error) => {
            eprintln!("Erro ao converter a resposta em HTML: {error}");
            return
        }
    };

    // parsear o html em busca dos preços (levar em consideração locais de preço com desconto e preço original)
    

}
