# ranks.db initializer

This tool initializes the "maps" table of an [osu! ranked lobbies](https://github.com/kiwec/osu-ranked-lobbies) database.

You should have ~117k beatmaps in osu! before running this. There are torrents available to get you started if you don't have them yet.

### Usage

```
git clone https://github.com/kiwec/orl-maps-db-generator && cd orl-maps-db-generator
cargo run --release <game directory>
```
