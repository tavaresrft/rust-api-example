use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long = "host", default_value = "0.0.0.0")]
    pub host: String,

    #[structopt(long = "port", default_value = "8080")]
    pub port: u32,   
    
    #[structopt(long = "model-path", default_value = "0.0.0.0")]
    pub model_path: String,
}

impl Cli {
    pub fn parse() -> Cli {
        Cli::from_args()
    }
}
