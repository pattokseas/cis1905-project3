use clap::{Parser, Subcommand};
use ngram::client::Client;
use ngram::message::Response;
use ngram::server::Server;

// Fill out the `Args` struct to parse the command line arguments. You may find clap "subcommands"
// helpful.
/// An archive service allowing publishing and searching of books
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Modes,
}

#[derive(Subcommand, Debug)]
enum Modes {
    Client {
        #[arg(default_value_t = String::from("127.0.0.1"))]
        address: String,
        #[arg(default_value_t = 7878)]
        port: u16,
        #[command(subcommand)]
        request: Requests,
    },
    Server {
        #[arg(default_value_t = 7878)]
        port: u16,
    },
}

#[derive(Subcommand, Debug)]
enum Requests {
    Publish { path: String },
    Search { word: String },
    Retrieve { id: usize },
}

// Inspect the contents of the `args` struct that has been created from the command line arguments
// the user passed. Depending on the arguments, either start a server or make a client and send the
// appropriate request. You may find it helpful to print the request response.
fn main() {
    let args = Args::parse();
    match args.mode {
        Modes::Client {
            address,
            port,
            request,
        } => {
            let client = Client::new(&address, port);
            let response = match request {
                Requests::Publish { path } => client.publish_from_path(&path),
                Requests::Search { word } => client.search(&word),
                Requests::Retrieve { id } => client.retrieve(id),
            };
            match response {
                None => println!("Server sent invalid result"),
                Some(Response::PublishSuccess(id)) => println!("PublishResponse({})", id),
                Some(Response::SearchSuccess(ids)) => {
                    print!("SearchResponse([");
                    for i in 0..ids.len() {
                        print!("{}", ids[i]);
                        if i + 1 < ids.len() {
                            print!(", ");
                        }
                    }
                    println!("])");
                }
                Some(Response::RetrieveSuccess(doc)) => println!("RetrieveResponse(\"{}\")", doc),
                Some(Response::Failure) => println!("Request failed"),
            };
        }
        Modes::Server { port } => {
            let server = Server::new();
            server.run(port);
        }
    };
}
