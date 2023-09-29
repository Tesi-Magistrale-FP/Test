# Test 3 e 4 - Scrittura e lettura di messaggi sui canali IOTA Streams

## Test 3 - Scrittura dei messaggi
Test in cui si effettua la scrittura sul canale Streams di 100 messaggi contenenti delle coordinate geografiche (latitudine, longitudine) generate casualmente. La scrittura avviene sia sulla Devnet sia sulla Mainnet, più nello specifico su un canale privato a singolo branch. Per ogni scrittura, viene misurato il tempo necessario per la creazione e la scrittura del messaggio sul canale.

## Test 4 - Lettura dei messaggi
Test in cui si effettua la lettura dal canale Streams dei 100 messaggi scritti precedentemente nel test 3. La lettura avviene sia sulla Devnet sia sulla Mainnet, più nello specifico su un canale privato a singolo branch. Vengono letti tutti i messaggi scritti, partendo dal messaggio di annuncio fino ad arrivare all'ultimo messaggi scritto da parte dell'autore del canale. Viene misurato il tempo necessario per leggere tutti i messaggi dal canale e accedere al loro contenuto.

### Riferimenti utili
- [Framework IOTA Streams](https://wiki.iota.org/streams/overview/)
- [Usare IOTA Streams](https://wiki.iota.org/streams/libraries/rust/getting_started/)
- [Esempio di canale privato a singolo branch](https://github.com/iotaledger/streams-examples/blob/master/src/examples/single_publisher/single_branch_private.rs)