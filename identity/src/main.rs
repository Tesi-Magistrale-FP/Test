use anyhow::Result;
use pretty_env_logger;

mod identity;

#[tokio::main]
async fn main() -> Result<()> 
{
    pretty_env_logger::init();

    println!("\nCrezione issuer\n");
    identity::identity::crea_issuer().await?;

    // Obtain a JSON representation of a credential issued to us
    println!("\n---------------------------------------");
    println!("\nCreazione utente\n");
    let credential_json: String = identity::identity::crea_verifiable_credentials(String::from("alice"), String::from("alice@gmail.com")).await?;

    println!("\n---------------------------------------");
    println!("\nValidazione credenziali utente\n");
    identity::identity::validazione_credenziali(credential_json).await?;

    println!();
    Ok(())
}