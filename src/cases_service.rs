use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CovidApiResponseForCountry {
    pub country: String,
    pub cases: i32,
    pub todayCases: i32,
}

pub async fn query_cases(
    country: String,
) -> Result<CovidApiResponseForCountry, Box<dyn std::error::Error>> {
    let base_url = String::from("https://corona.lmao.ninja/countries/");
    let url = format!("{}{}", base_url, country);

    let resp = reqwest::get(&url)
        .await?
        .json::<CovidApiResponseForCountry>()
        .await?;

    Ok(resp)
}
