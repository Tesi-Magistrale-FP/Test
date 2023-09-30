# Test 7 e 8 - Scrittura e lettura su database

## Test 7 - Scrittura e lettura di messaggi semplici
Test in cui si effettua la scrittura e la lettura su un database MongoDB di 100 messaggi contenenti delle coordinate geografiche (latitudine, longitudine) generate casualmente. Per ogni scrittura, viene misurato il tempo necessario per preparare e comunicare al database il messaggio. Per ogni lettura, viene misurato il tempo necessario per ottenere il dato e accedere al suo contenuto.

## Test 8 - Scrittura e lettura di messaggi cifrati e autenticati
Test in cui si effettua la scrittura e la lettura su un database MongoDB di 100 messaggi contenenti delle coordinate geografiche (latitudine, longitudine) generate casualmente. Ogni messaggio è cifrato con il cifrario simmetrico AES-GCM ed è firmato digitalmente con il cifrario asimmetrico Ed25519. Per ogni scrittura, viene misurato il tempo necessario per preparare, firmare, cifrare e comunicare al database il messaggio. Per ogni lettura, viene misurato il tempo necessario per ottenere il dato, decifrarlo, verificare la firma e accedere al suo contenuto.

### Riferimenti utili
- [MongoDB](https://www.mongodb.com/it-it/)
- [AES-GCM - Documentazione](https://docs.rs/aes-gcm/latest/aes_gcm/)
- [Ed25519 - Documentazione](https://docs.rs/ed25519-dalek/latest/ed25519_dalek/)