use std::env;
use std::path::Path;
use osu_db::listing::{Listing};
use rosu_pp::{Beatmap, OsuPP};
use sqlite::Value::{Float, Integer, String as VString};


fn main() {
    let args: Vec<String> = env::args().collect();
    let game_directory = Path::new(&args[1]);
    let songs_directory = game_directory.join("Songs");

    let connection = sqlite::open("maps.db").unwrap();
    connection.execute(
        "CREATE TABLE map (
            id INTEGER PRIMARY KEY,
            set_id INTEGER NOT NULL,
            name TEXT NOT NULL,

            length REAL NOT NULL,
            ranked INT NOT NULL,
            dmca INTEGER NOT NULL,

            stars REAL NOT NULL,
            aim_pp REAL NOT NULL,
            speed_pp REAL NOT NULL,
            acc_pp REAL NOT NULL,
            overall_pp REAL NOT NULL,
            ar REAL NOT NULL,

            dt_stars REAL NOT NULL,
            dt_aim_pp REAL NOT NULL,
            dt_speed_pp REAL NOT NULL,
            dt_acc_pp REAL NOT NULL,
            dt_overall_pp REAL NOT NULL,
            dt_ar REAL NOT NULL
        )"
    ).unwrap();

    // Avoid writing to disk since this is a one-time job
    connection.execute("PRAGMA synchronous=OFF").unwrap();
    connection.execute("PRAGMA count_changes=OFF").unwrap();
    connection.execute("PRAGMA journal_mode=MEMORY").unwrap();
    connection.execute("PRAGMA temp_store=MEMORY").unwrap();
    connection.execute("BEGIN TRANSACTION").unwrap();

    let mut insert = connection
        .prepare(
            "INSERT INTO map (
                id, set_id, name, length, ranked, dmca,
                stars, ar, aim_pp, speed_pp, acc_pp, overall_pp,
                dt_stars, dt_ar, dt_aim_pp, dt_speed_pp, dt_acc_pp, dt_overall_pp
            ) VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .unwrap()
        .cursor();

    let db_file = game_directory.join("osu!.db");
    let mut listing = Listing::from_file(db_file).unwrap();

    let nb_maps = listing.beatmaps.len();
    let mut i = 1;

    for beatmap in listing.beatmaps.iter_mut() {
        let map_name = beatmap.title_ascii.as_ref().unwrap();
        println!("Processing beatmap {}/{} ({})", i, nb_maps, map_name);

        let map_folder = beatmap.folder_name.as_ref().unwrap();
        let map_filename = beatmap.file_name.as_ref().unwrap();


        let map = match Beatmap::from_path(songs_directory.join(map_folder).join(map_filename)) {
            Ok(val) => val,
            Err(e) => {
                println!("Skipped map due to error: {}", e);
                continue;
            }
        };

        let nm_pp = OsuPP::new(&map).calculate();
        let dt_pp = OsuPP::new(&map).mods(1<<6).calculate();

        insert.bind(&[
            Integer(beatmap.beatmap_id as i64),
            Integer(beatmap.beatmapset_id as i64),
            VString(map_name.to_string()),
            Integer(beatmap.drain_time as i64),
            Integer(beatmap.status.raw() as i64),
            Float(nm_pp.difficulty.stars),
            Float(nm_pp.difficulty.ar),
            Float(nm_pp.pp_aim),
            Float(nm_pp.pp_speed),
            Float(nm_pp.pp_acc),
            Float(nm_pp.pp),
            Float(dt_pp.difficulty.stars),
            Float(dt_pp.difficulty.ar),
            Float(dt_pp.pp_aim),
            Float(dt_pp.pp_speed),
            Float(dt_pp.pp_acc),
            Float(dt_pp.pp)
        ]).unwrap();

        insert.next().unwrap();
        i = i + 1;
    }

    connection.execute("COMMIT TRANSACTION").unwrap();
}
