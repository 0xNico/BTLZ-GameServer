<img align="center" width="100" src="https://i.imgur.com/cBpNT5V.png" alt="BTLZ Weapon">

### BTL-Z An Idle RPG & Meta Social Collectors Game  ⚔️

BattleZ is a Fast-Paced Idle RPG based on the concept of **opportunity** rather than the typical grind that you see in most RPG genre games. Every 24H the game will emit events to all players in the form of battles. Players should take these opportunities to gain xp, collect loot and level their account.

***Core Game Loop of BattleZ***

* Players collect a ZERO by ZKVerse avatar and create a Stats Account for this NFT.
* Players then participate in **opportunity** based battle events to gain XP, loot and level up.
* Players can then convert loot to **rare Classes and Weapons** which **can be traded.**
* Players can **accrue value through multiple methods** - Lvl up NFTs or collecting Classes and Weapons.
* *Events, Rare Releases and Hard Farm Collectibles stimulate consistent economic movement.*




---

#### Technical Details for Developers 👨‍💻

- The "BTL-Z" Game Server is written entirely in the **Rust** programming language.
- It utilises Actix web for *blazing fast* concurrent server interactions and **sub ms speeds.**

     **Running Locally.**

```rust
cargo build // obviously.
cargo run --bin btlz_rust // runs the main BattleZ game server.
cargo run --bin instance_bench // benchmark creating 1000 battles and joining with 1000 clients.

```

