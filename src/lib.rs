pub struct Config {
    query: String,
    file_path: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
