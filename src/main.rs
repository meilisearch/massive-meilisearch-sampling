use std::{ffi::OsStr, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use percentage::Percentage;
use rusqlite::{Connection, Row};

mod percentage;

/// A program that generates and sends dataset and
/// sample update/deletes to a Meilisearch server.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The dataset that will be used to do the operations.
    #[arg(long)]
    dataset: PathBuf,

    /// The percentage of the dataset to delete at each operation.
    #[arg(long, default_value = "10%")]
    deletes: Percentage,

    /// The percentage of the dataset to update at each operation.
    #[arg(long, default_value = "20%")]
    updates: Percentage,

    /// The database file in which to load the dataset.
    #[arg(short, long, default_value = "database.sqlite")]
    database: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Args {
        dataset,
        deletes,
        updates,
        database,
    } = Args::parse();

    let sqlite_conn = Connection::open(database)?;

    // We first check if the song table exists and is empty or not.
    if sqlite_conn.execute("SELECT count(*) FROM sqlite_master WHERE name='song'", ())? == 0 {
        sqlite_conn.execute(include_str!("../song.schema"), ())?;

        anyhow::ensure!(
            dataset.extension() == Some(OsStr::new("csv")),
            "Only CSV files are supported for now"
        );

        let mut rdr = csv::Reader::from_path(&dataset)
            .with_context(|| format!("while opening `{}`", dataset.display()))?;

        for result in rdr.deserialize() {
            let song: Song = result?;
            sqlite_conn.execute(
                "INSERT INTO song
                       (id, title, album, artist, genre, country, released, duration, released_timestamp, duration_float)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                (&song.id,
                &song.title,
                &song.album,
                &song.artist,
                &song.genre,
                &song.country,
                &song.released,
                &song.duration,
                &song.released_timestamp,
                &song.duration_float)
            )?;
        }
    }

    let mut stmt = sqlite_conn.prepare("SELECT * FROM song ORDER BY RANDOM() LIMIT :count")?;
    let song_iter = stmt.query_map(&[(":count", &"2")], Song::from_complete_row)?;

    println!("2 random songs:");
    for song in song_iter {
        println!("{:?}", song.unwrap());
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Song {
    pub id: u64,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub country: String,
    pub released: String,
    pub duration: String,
    pub released_timestamp: Option<u64>,
    pub duration_float: Option<f32>,
}

impl Song {
    fn from_complete_row(row: &Row<'_>) -> rusqlite::Result<Song> {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            album: row.get(2)?,
            artist: row.get(3)?,
            genre: row.get(4)?,
            country: row.get(5)?,
            released: row.get(6)?,
            duration: row.get(7)?,
            released_timestamp: row.get(8)?,
            duration_float: row.get(9)?,
        })
    }
}
