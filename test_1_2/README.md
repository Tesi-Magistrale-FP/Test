# Test 1 e 2 - Scrittura e lettura di transazioni semplici sul Tangle

## Test 1 - Scrittura dei messaggi
Test in cui si effettua la scrittura sul Tangle di 100 messaggi contenenti delle coordinate geografiche (latitudine, longitudine) generate casualmente. La scrittura avviene sia sulla Devnet sia sulla Mainnet. Per ogni scrittura, viene misurato il tempo necessario per la creazione e la scrittura della transazione sul Tangle.

## Test 2 - Lettura dei messaggi
Test in cui si effettua la lettura dal Tangle dei 100 messaggi scritti precedentemente nel test 1. La lettura avviene per ogni singolo messaggio, sfruttando il suo indirizzo. La lettura avviene sia sulla Devnet sia sulla Mainnet. Per ogni lettura, viene misurato il tempo necessario per leggere la transazione dal Tangle e accedere al suo contenuto.

### Riferimenti utili
- [Scrivere un messaggio dati](https://wiki.iota.org/iota.rs/examples/data_message/)
- [Leggere un messaggio dati](https://wiki.iota.org/iota.rs/examples/get_message_data/)
- [Struttura del messaggio](https://wiki.iota.org/iota.rs/explanations/messages_payloads_and_transactions)