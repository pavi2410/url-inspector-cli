use url::Url;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    url: Option<String>,
    part: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(url) = args.url.as_deref() {
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(part) = args.part.as_deref() {
                let r = match part {
                    "scheme" => parsed_url.scheme().to_string(),
                    "host" => parsed_url.host_str().unwrap().to_string(),
                    "port" => parsed_url.port().unwrap().to_string(),
                    "path" => parsed_url.path().to_string(),
                    "fragment" => parsed_url.fragment().unwrap().to_string(),
                    _ => "Err: Invalid part".to_string(),
                };
                println!("{}", r);
            } else {
                // print all the parts
                println!("scheme: {}", parsed_url.scheme());
                println!("host: {}", parsed_url.host_str().unwrap());
                println!("port: {}", parsed_url.port().unwrap());
                println!("path: {}", parsed_url.path());
                println!("fragment: {}", parsed_url.fragment().unwrap());
            }
        } else {
            println!("Err: Invalid URL");
        }
    } else {
        println!("Err: No URL provided");
    }
}
