use anyhow::Result;
use kvdb_rocksdb::*;
use std::{
    fs,
    io::{Read as _, Write as _},
    path::PathBuf,
};
use structopt::StructOpt;
use thiserror::Error;

/// Parity 2.5.13 db model version
const PARITY_2_5_13_VERSION: u32 = 13;
/// Parity 2.7.2 db model version
const PARITY_2_7_2_VERSION: u32 = 14;
/// OpenEthereum 3.0.1 db model version
const OE_3_0_1_VERSION: u32 = 15;
/// OpenEthereum 3.1.0 db model version
const OE_3_1_0_VERSION: u32 = 16;

const VERSION_FILE_NAME: &str = "db_version";
const BATCH_SIZE: usize = 2500;

pub const COL_ACCOUNT_BLOOM: u32 = 5;

pub const AGREEMENT_SENTENCE: &str = "I AGREE";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Migration cancelled.")]
    Cancelled,
    #[error("Current database version cannot be read. Check if folder exists.")]
    UnknownDatabaseVersion(String),
    #[error("Current database version `{0}` is not supported")]
    UnsupportedDatabaseVersion(u32),
    #[error("Unexpected io error on DB migration")]
    Io(#[from] std::io::Error),
}

#[derive(StructOpt)]
#[structopt(
    name = "oe-update-db-3-1",
    about = "OpenEthereum 2.5.13, 2.7.2, 3.0.1 to 3.1/3.2 DB upgrade tool"
)]
struct Cli {
    /// The path to the db folder.
    /// Example: `~/.local/share/io.parity.ethereum/chains/ethereum/db/906a34e69aec8c0d/overlayrecent`.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn version_file_path(mut path: PathBuf) -> PathBuf {
    path.push(VERSION_FILE_NAME);
    path
}

fn update_version(path: PathBuf) -> Result<(), Error> {
    fs::create_dir_all(&path)?;
    let mut file = fs::File::create(version_file_path(path))?;
    file.write_all(format!("{}", OE_3_1_0_VERSION).as_bytes())?;
    Ok(())
}

fn current_version(path: PathBuf) -> Result<u32, Error> {
    match fs::File::open(version_file_path(path)) {
        Err(err) => Err(Error::UnknownDatabaseVersion(format!(
            "cannot open: {:?}",
            err
        ))),
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s).map_err(|err| {
                Error::UnknownDatabaseVersion(format!("cannot read version: {:?}", err))
            })?;
            u32::from_str_radix(&s, 10).map_err(|err| {
                Error::UnknownDatabaseVersion(format!("cannot parse version: {:?}", err))
            })
        }
    }
}

fn database_path(mut path: PathBuf) -> PathBuf {
    path.push("db");
    path
}

fn remove_accounts_bloom(db: &Database) -> Result<(), Error> {
    loop {
        let mut transaction = db.transaction();
        db.iter(COL_ACCOUNT_BLOOM)
            .take(BATCH_SIZE)
            .for_each(|(k, _)| transaction.delete(COL_ACCOUNT_BLOOM, &k));

        if transaction.ops.is_empty() {
            break;
        }
        db.write(transaction)?;
    }
    Ok(())
}

fn migrate_database(db_path: PathBuf) -> Result<(), Error> {
    // check if a migration is needed
    let current_version = current_version(db_path.clone())?;

    let (columns, display) = match current_version {
        r#PARITY_2_5_13_VERSION => (8, "2.5"),
        r#PARITY_2_7_2_VERSION => (9, "2.7"),
        r#OE_3_0_1_VERSION => (9, "3.0"),
        r#OE_3_1_0_VERSION => {
            println!("Already in last version. No migration needed");
            return Ok(());
        }
        _ => return Err(Error::UnsupportedDatabaseVersion(current_version)),
    };

    println!(
        "Migrating database from v{} ({} series) to v16 (3.1/3.2 series).",
        current_version, display
    );

    let mut i_agree = String::new();
    println!(
        "CAUTION: This will update the database in a non reversible way. A backup is recommended. "
    );
    print!("type {} to proceed > ", AGREEMENT_SENTENCE);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut i_agree)?;
    if i_agree.trim() != AGREEMENT_SENTENCE {
        return Err(Error::Cancelled);
    }

    println!("- opening the database...");
    let db = kvdb_rocksdb::Database::open(
        &DatabaseConfig::with_columns(columns),
        &database_path(db_path.clone()).to_string_lossy(),
    )?;

    match current_version {
        r#PARITY_2_5_13_VERSION => {
            println!("- removing accounts bloom.");
            remove_accounts_bloom(&db)?;
            println!("- removing light chain data.");
            db.remove_last_column()?; // COL_LIGHT_CHAIN
        }
        r#PARITY_2_7_2_VERSION => {
            println!("- removing accounts bloom.");
            remove_accounts_bloom(&db)?;
            println!("- removing light chain data.");
            db.remove_last_column()?; // COL_LIGHT_CHAIN
            println!("- removing private transaction data.");
            db.remove_last_column()?; // COL_PRIVATE_TRANSACTIONS_STATE
        }
        r#OE_3_0_1_VERSION => {
            println!("- removing light chain data.");
            db.remove_last_column()?; // COL_LIGHT_CHAIN
            println!("- removing private transaction data.");
            db.remove_last_column()?; // COL_PRIVATE_TRANSACTIONS_STATE
        }
        _ => unreachable!(),
    }

    // update version file.
    println!("- updating version.");
    update_version(db_path)?;
    println!("- migration completed.");

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    migrate_database(args.path)
}
