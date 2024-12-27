use clap::Parser;
use log::info;
use offload_lib::get_dir_size;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    /// Code base directory path
    #[arg(short, long, default_value_t = String::from("."))]
    path: String, 

    /// Scans directories to report used storage
    #[arg(short, long, default_value_t = String::from("no"))]
    scan: String, 

    /// Programming language used 
    #[arg(short, long, default_value_t = String::from("rust"))]
    lang: String
}


fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Trace) // Set default to Trace
        .init();

    let args = Args::parse();
    let mut _path : String = args.path;

    info!("Code base located at {}", _path);

    let y = get_dir_size("Cargo.toml".to_string());
    println!("The size of dir is {:?}", y);
}


