use chrono::NaiveDateTime;
use postgres::{Client, Config, NoTls};
use time::PrimitiveDateTime;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.host("/var/run/postgresql");
    let mut client = config.connect(NoTls)?;

    // OK
    inf_test(&mut client, "date")?;

    // panics
    inf_test(&mut client, "timestamp")?;
    inf_test(&mut client, "timestamptz")?;

    Ok(())
}


fn inf_test(client: &mut Client, ty: &str) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("SELECT 'infinity'::{ty}");

    println!("Testing {ty}");

    let row = client.query_one(&query, &[])?;

    println!("chrono");

    // `chrono` overflow handled
    let chrono_err = row.try_get::<_, NaiveDateTime>(0).unwrap_err();
    eprintln!("{ty} with chrono: {chrono_err}");

    println!("time");

    // `time` overflow panics
    match row.try_get::<_, PrimitiveDateTime>(0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{ty} with time: Error: {e}"),
    }

    println!("\n");
    Ok(())
}