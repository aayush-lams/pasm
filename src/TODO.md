- make serializationa and deserialization not panic

- skip the loop instead of paniking if decription error in list api

- add methods to update the encryptino key if verified
  - basically if authorized, user can update the encryption_key so that all
    entries are decrypted with old encr_key and then rewritten with new new_encr_key
  - then old_encr_key is replaced by new_encr_key

- make robust client side with public dns connection

- currently only works on localhost, but will use 0.0.0.0 to allow public device to connect
  - should implement proper firewall and hardening to make it robust

- crypto module
- better authentication
- better database layer, prolly redis
- audit logging
- rate limiting
- CLI client
- docker-compose
- security documentation auto gen
