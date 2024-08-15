use chrono::NaiveDateTime;
use postgres::{Config, NoTls};
use time::PrimitiveDateTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.host("/var/run/postgresql");
    let mut client = config.connect(NoTls)?;

    let row = client.query_one(
        "SELECT 'infinity'::timestamp",
        &[],
    )?;

    // `chrono` overflow handled
    let chrono_err =  row.try_get::<_, NaiveDateTime>(0).unwrap_err();
    eprintln!("{chrono_err}");

    // `time` overflow panics
    match row.try_get::<_, PrimitiveDateTime>(0) {  // this panics
        Ok(_) => unreachable!(),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}