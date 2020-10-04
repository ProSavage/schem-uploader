A replacement for schematica/printer mod.
## Schem Uploader
A web service to handle player's schematic data by linking minecraft accounts, and providing a web interface for 
managing said schematics, with a minecraft plugin that allows players to pay, then, print these schematics in game.

### Backend
This system needs to be able to link minecraft accounts to the service.
We can use `UUID` from the minecraft players, but it has a few points of failure:
- UUID spoofing exists, so we cannot directly trust a logged in player to be genuine.
- Offline servers do not have premium uuids, so we need to somehow authenticate that too.

**Auth Flow**
1. Open web portal. 
2. Hit `login` endpoint to attempt login
    - Failure IF the player does not have their UUID registered.
    - Failure IF the player does not know their UUID's password.
3. If failure, hit `signup` endpoint, and attempt to link in-game UUID w/ a password.
    - This will need to instruct players to join an official server, to make sure the UUID is genuine.
       1. Finish signup, then get a code ex: `Cool-Underwater-Sheep`.
       2. Join official schem-uploader authentication minecraft server, and put code in chat.
       3. If successful, link back to webpanel ingame, signup is complete.
    - Failure IF the UUID is already registered.

**Upon Login**
1. Login with linked Schematic Uploader Account.
2. Create, store, and send auth token for UUID to web client.
3. Redirect to schematic management UI.