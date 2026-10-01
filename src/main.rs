

mod http_client;
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
            eprintln!("Erro ao fazer requisição para a url '{pichau_url}':\n{error}");
            return
        }
    };

    let response_status_code = pichau_response.status();

    // dar log no status
    println!("Status code: {response_status_code}");
}
