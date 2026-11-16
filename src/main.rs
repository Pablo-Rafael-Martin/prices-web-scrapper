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

    // chamar a url da pichau
    let pichau_url = "https://www.pichau.com.br/search?q=rx 9070 xt";

    println!("Buscando na Pichau...");
    let pichau_response = match client.new_request(pichau_url).await {
        Ok(response) => {
            println!("Busca na Pichau concluída.");
            response
        }
        Err(error) => {
            // ! SE DÁ ERRO ELE RETORNA, TEM QUE PENSAR EM COMO TRATAR ERRO SEM ENCERRAR A EXECUÇÃO

            eprintln!("Erro ao fazer requisição para a url '{pichau_url}':\n{error}");
            return;
        }
    };

    {
        // dar log no status
        let response_status_code = pichau_response.status();
        println!("Status code: {response_status_code}");
    }

    // converter a resposta pra texto e parsear o html
    let pichau_response_text = match pichau_response.text().await {
        Ok(result) => {
            println!("Html convertido para texto com sucesso.");
            result
        }
        Err(error) => {
            eprintln!("Erro ao converter a resposta em HTML: {error}");
            return;
        }
    };

    let response_html = Html::parse_document(&pichau_response_text);

    // parsear o html em busca dos preços (levar em consideração locais de preço com desconto e preço original)
    let mut product_card_list =
        match html_parser::parse_html(&response_html, "a[data-cy=\"list-product\"]") {
            Ok(result) => {
                println!("Produtos encontrados.");
                result
            }
            Err(error) => {
                eprintln!("Erro ao encontrar os produtos na pichau: {error}");
                return;
            }
        };

    {
        let products_found = product_card_list.len();
        println!("Número de produtos encontrados: {products_found}");
    }

    // selecionar os cards que contêm o texto "placa de video" no título
    product_card_list.retain(|product| {
        let product_to_html = Html::parse_document(&product.html());

        let parsed_product = match parse_html(&product_to_html, "h2") {
            Ok(non_error) => non_error,
            Err(error) => {
                eprintln!("Erro inesperado ao procurar o título do produto: {error}");
                return false;
            }
        };

        let condition = parsed_product[0]
            .inner_html()
            .trim()
            .to_lowercase()
            .contains("placa de video");

        return condition;
    });

    // montar uma struct Product contendo os dados do resultado e dar log nesses produtos
}
