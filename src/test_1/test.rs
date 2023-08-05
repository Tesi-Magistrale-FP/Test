use iota_client::Client;
use iota_client::Result;
use rand::Rng;

pub async fn test_tx_dev() -> Result<()>
{
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    let lat = rand::thread_rng().gen_range(-90.0..=90.0);
    let lon = rand::thread_rng().gen_range(-180.0..=180.0);

    let message = iota
        .message()
        .with_index("TrackApp")
        .with_data(format!("({}, {})", lat, lon).as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/devnet/message/{}\n",
        message.id().0
    );

    let fetched_message_ids = iota.get_message().index("TrackApp").await.unwrap();
    println!("Messages with TrackApp index: {fetched_message_ids:?}");

    Ok(())
}

pub async fn test_tx_main() -> Result<()>
{
    let iota = Client::builder()
        .with_node("https://chrysalis-nodes.iota.org")?
        .finish()
        .await?;

    let lat = rand::thread_rng().gen_range(-90.0..=90.0);
    let lon = rand::thread_rng().gen_range(-180.0..=180.0);

    let message = iota
        .message()
        .with_index("TrackApp")
        .with_data(format!("({}, {})", lat, lon).as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/mainnet/message/{}\n",
        message.id().0
    );

    let fetched_message_ids = iota.get_message().index("TrackApp").await.unwrap();
    println!("Messages with TrackApp index: {fetched_message_ids:?}");

    Ok(())
}