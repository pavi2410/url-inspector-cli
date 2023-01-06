use clap::{Parser, ValueEnum};
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    url: String,

    #[arg(value_enum)]
    part: Option<Part>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    Scheme,
    Username,
    Password,
    Host,
    Port,
    Path,
    Query,
    Fragment,
}

fn main() {
    let args = Cli::parse();

    match Url::parse(args.url.as_str()) {
        Ok(parsed_url) => {
            if let Some(part) = args.part {
                match part {
                    Part::Scheme => println!("{}", parsed_url.scheme()),
                    Part::Username => println!("{}", parsed_url.username()),
                    Part::Password => println!("{}", parsed_url.password().unwrap_or_default()),
                    Part::Host => println!("{}", parsed_url.host_str().unwrap()),
                    Part::Port => println!("{}", parsed_url.port_or_known_default().unwrap_or_default()),
                    Part::Path => println!("{}", parsed_url.path()),
                    Part::Query => println!("{}", parsed_url.query().unwrap_or_default()),
                    Part::Fragment => println!("{}", parsed_url.fragment().unwrap()),
                };
            } else {
                // print all the parts
                println!("scheme: {}", parsed_url.scheme());
                println!("username: {}", parsed_url.username());
                println!("password: {}", parsed_url.password().unwrap_or_default());
                println!("host: {}", parsed_url.host_str().unwrap_or_default());
                println!("port: {}", parsed_url.port_or_known_default().unwrap_or_default());
                println!("path: {}", parsed_url.path());
                println!("query: {}", parsed_url.query().unwrap_or_default());
                println!("fragment: {}", parsed_url.fragment().unwrap_or_default());
            }
        }
        Err(e) => {
            println!("Err: {}", e);
        }
    }
}
