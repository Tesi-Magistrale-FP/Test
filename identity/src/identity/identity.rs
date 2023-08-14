use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

use identity_iota::iota_core::IotaDID;
use identity_iota::account::Account;
use identity_iota::account::IdentitySetup;
use identity_iota::account::Result;
use identity_iota::account::MethodContent;
use identity_iota::client::CredentialValidationOptions;
use identity_iota::client::CredentialValidator;
use identity_iota::client::FailFast;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::Subject;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::DID;
use identity_iota::account_storage::Stronghold;
use identity_iota::client::ExplorerUrl;

static mut ISSUER: Option<Account> = None;
static mut HOLDERS: Option<HashMap<String, String>> = None;

// Funzione per creare l'Issuer, ovvero l'entità che asserisce affermazioni su un soggetto
pub async fn crea_issuer() -> Result<()>
{
    unsafe
    {
        // Stronghold è un file cifrato che gestisce le chiavi private. Viene implementato usando le migliori pratiche di sicurezza ed è raccomandato per gestire le chiavi private
        let stronghold_path: PathBuf = ".stronghold/issuer-strong.hodl".into();                        	// Path in cui verrà salvato il file Stronghold
        let password: String = "issuer_pwd".to_owned();                                                 // Password dell'Issuer per sbloccare e accedere ai dati del suo file Stronghold
        let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;         	// Creazione Stronghold

        // Creazione dell'identità decentralizzata dell'Issuer, usando il file Stronghold come storage locale
        // La creazione si basa sulla generazione di una coppia di chiavi, sulla costruzione di un'identità e sulla sua pubblicazione nella Mainnet IOTA
        ISSUER = Some(Account::builder()
            .storage(stronghold)
            .create_identity(IdentitySetup::default())
            .await?);

        // Aggiunge all'Issuer il metodo per effettuare la verifica delle Verifiable Credentials
        ISSUER.as_mut().unwrap()
            .update_identity()
            .create_method()
            .content(MethodContent::GenerateEd25519)
            .fragment("issuer_pwd")
            .apply()
            .await?;

        // Restituisce il valore del DID associato all'identità appena creata 
        let iota_did: &IotaDID = ISSUER.as_mut().unwrap().did();

        println!("- Crezione Issuer avvenuta con successo");

        // Mostra lo stato locale del Documento DID
        println!("- Documento generato associato alla DID {} \n{:#?}", iota_did, ISSUER.as_mut().unwrap().document());

        // Mostra l'URL dell'Identity Resolver Explorer che consente di vedere il Documento DID online e tutta la sua storia passata
        let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
        println!("\n- Documento DID disponibile al seguente link -> {}", explorer.resolver_url(iota_did)?);

        HOLDERS = Some(HashMap::new());                                               					// Inizializzo la struttura che conterrà tutti gli Holders, ovvero gli utenti con delle Verifiable Credentials
    }

    Ok(())
} 

// Funzione per creare le credenziali (Verifiable Credentials) da asssociare a un Holders (entità che possiede delle VC), ovvero a un utente che vorrà usare il servizio
pub async fn crea_verifiable_credentials(username: String, email: String) -> Result<String> 
{
    // Crea un'identità per l'Holder, che in questo caso è anche il Subject (entità su cui vengono fatte delle affermazioni)
    let user: Account = Account::builder().create_identity(IdentitySetup::default()).await?;

    // Restituisce il valore del DID associato all'identità appena creata
    let user_did: &IotaDID = user.did();

    println!("- Crezione utente {username} avvenuta con successo");

    // Mostra lo stato locale del Documento DID
    println!("- Documento generato associato alla DID {} \n{:#?}", user_did, user.document());

    // Mostra l'URL dell'Identity Resolver Explorer che consente di vedere il Documento DID online e tutta la sua storia passata
    let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
    println!("\n- Documento DID disponibile al seguente link -> {}", explorer.resolver_url(user_did)?);

    // Crea una credenziale, da associare al Subject, che contiene delle informazioni utili per la piattaforma BlocksShare
    let subject: Subject = Subject::from_json_value(json!({
        "id": user_did,
        "username": username,
        "email": email,
        "GPA": "4.0",
    }))?;
    
    unsafe
    {
        // Ottiene delle credenziali a partire dal Subject e dall'Issuer
        let mut credenziali: Credential = CredentialBuilder::default()
            .id(Url::parse(format!("https://blocksshare.com/credentials/{user_did}"))?)
            .issuer(Url::parse(ISSUER.as_mut().unwrap().did().as_str())?)
            .type_("BlocksShareCredential")
            .subject(subject)
            .build()?;
        
        // Firma le credenziali create con il metodo di verifica dell'Issuer
        ISSUER.as_mut().unwrap()
            .sign("#issuer_pwd", &mut credenziali, ProofOptions::default())
            .await?;

        // Prima di inviare le credenziali all'Holder, l'Issuer deve controllare che alcune proprietà delle credenziali rispettino le aspettative.
        // Questo controllo avviene tramite la seguente validazione.
        
        // Il controllo si concentra su:
        // - La verifica della firma delle credenziali usando il Documento DID dell'Issuer
        // - La correttezza della struttura semantica delle credenziali
        // - La validità della data di emissione (non deve essere una data futura)
        // - La validità della data di scadenza (non deve essere una data passata)
        CredentialValidator::validate(
            &credenziali,
            &ISSUER.as_mut().unwrap().document(),
            &CredentialValidationOptions::default(),
            FailFast::FirstError,
        )
        .unwrap();
        
        println!("- Credenziali validate con successo");

        // L'Issuer ora è sicure che le credenziali soddisfano le aspettative.
        // Le credenziali ora possono essere serializzate in formato JSON e trasmesse all'utente in maniera sicura.
        // NB: Le credenziali non vengono pubblicate sul Tangle di IOTA, ma inviate e salvate off-chain
        let credenziali_json: String = credenziali.to_json()?;

        println!("- Credenziali JSON = \n{:#}", credenziali);

        // Aggiunta dell'utente, identificato tramite il suo DID, e delle sue credenziali all'interno dell'HashMap
        HOLDERS.as_mut().unwrap().insert(format!("{user_did}"), credenziali_json.clone());

        Ok(credenziali_json.clone())
    }
}

// Funzione usata per controllare la validità delle Verifiable Credentials e della loro associazione all'Holder (utente) di appartenenza
pub async fn validazione_credenziali(credenziali_json: String) -> Result<()>
{
    // Ottengo le credenziali dalla credenziali inviate dall'utente in formato JSON
    let credenziali: Credential = Credential::from_json(&credenziali_json)?;

    println!("- Credenziali JSON = {:#}", credenziali);

    unsafe
    {
        // Controllo della validità delle credenziali effettuato dall'Issuer.
        // Il controllo si concentra su:
        // - La verifica della firma delle credenziali usando il Documento DID dell'Issuer
        // - La correttezza della struttura semantica delle credenziali
        // - La validità della data di emissione (non deve essere una data futura)
        // - La validità della data di scadenza (non deve essere una data passata)
        CredentialValidator::validate(
            &credenziali,
            &ISSUER.as_mut().unwrap().document(),
            &CredentialValidationOptions::default(),
            FailFast::FirstError,
        )
        .unwrap();
        
        println!("- Credenziali validate con successo");

        // Analisi delle credenziali in formato JSON per estrarre il DID dell'Holder (utente)
        let root: Value = serde_json::from_str(&credenziali_json).unwrap();
        let user_did: Option<&str> = root.get("credentialSubject").and_then(|value| value.get("id")).and_then(|value| value.as_str());
    
        println!("- DID dell'utente = {}", String::from(user_did.unwrap()));

        // Leggo i valori all'interno dell'HashMap, usando il DID come chiave, per ottenere le credenziali associate a un certo Holder (utente)
        if String::from(HOLDERS.as_mut().unwrap().get(user_did.unwrap()).unwrap()) == credenziali_json 			// Se le credenziali inviate e i dati salvati corrispondono
        {
            println!("- Accesso consentito");
        }
        else                                                                                            		// Se i dati non corrispondono
        {
            println!("- Accesso non consentito");
        }
    }

    Ok(())
}