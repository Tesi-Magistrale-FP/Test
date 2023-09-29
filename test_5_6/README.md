# Test 5 e 6 - Scrittura e lettura di messaggi sugli IOTA Smart Contracts

## Test 5 - Scrittura dei messaggi
Test in cui si effettua la scrittura su uno ISC di 100 messaggi contenenti delle coordinate geografiche (latitudine, longitudine) generate casualmente. Il valore viene memorizzato nello stato interno dello smart contract. Per ogni scrittura, viene misurato il tempo necessario per eseguire il comando wasp-cli che si occupa della scrittura sullo ISC.

## Test 6 - Lettura dei messaggi
Test in cui si effettua la lettura dallo ISC dei 100 messaggi scritti precedentemente nel test 5. La lettura avviene dallo stato interno dello smart contract. Viene misurato il tempo necessario per leggere ogni messaggio dallo ISC e accedere al loro contenuto.

### Riferimenti utili
- [IOTA Smart Contracts](https://wiki.iota.org/learn/smart-contracts/introduction/)
- [Comunicazioni con gli ISC](https://wiki.iota.org/learn/smart-contracts/invocation/)
- [ISC usato](https://github.com/Tesi-Magistrale-FP/Nodo_Wasp/tree/main/contracts/wasm/performance)