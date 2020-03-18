mod cases_service;

use std::env::var;

use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("covid-19")
        .version("0.1")
        .author("Lukasz Gmys <lgmys@pm.e>")
        .about("Fetches Covid-19 cases count")
        .arg(
            Arg::with_name("simple")
                .short("s")
                .long("simple")
                .value_name("simple")
                .help("print output with simpler format: total(today cases)")
                .takes_value(false),
        )
        .get_matches();

    let country = var("COVID19_COUNTRY").unwrap_or(String::from("poland"));

    let resp = cases_service::query_cases(country).await?;

    if matches.is_present("simple") {
        println!("{}({})", resp.cases, resp.todayCases);
        return Ok(())
    }

    println!(
        "covid-19 cases in {}: {} ({} today)",
        resp.country, resp.cases, resp.todayCases
    );
    Ok(())
}
