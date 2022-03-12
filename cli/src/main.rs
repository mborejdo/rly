use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Michael Borejdo <mib@electronic-minds.de>")]
struct Opts {
    #[clap(short, long)]
    url: String
}

#[tokio::main]
async fn api_request(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let params = [("url", url)];
    let client = reqwest::Client::new();
    let res = client.post("https://rly.eminds.de")
        .form(&params)
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}

fn main() {
    let opts: Opts = Opts::parse();
    let short = api_request(opts.url.clone()).unwrap();
    println!("{}", short);
}