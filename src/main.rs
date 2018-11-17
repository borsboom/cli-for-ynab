extern crate cli_for_ynab;
extern crate dirs;
extern crate dotenv;
extern crate openssl_probe;

const DOT_ENV_FILE: &str = ".ynab.env";
const ENV_FILE: &str = "ynab.env";

fn dotenv_err<T>(prog_name: &str, env_path: String, result: Result<T, dotenv::Error>) {
    if let Err(e) = result {
        if !e.not_found() {
            // @@@ CHECK FORMATTING
            eprintln!("{} error: {}: {}", prog_name, env_path, e);
            std::process::exit(1);
        }
    }
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let prog_name = clap::App::new("")
        .get_bin_name()
        .unwrap_or("ynab")
        .to_owned();
    // @@@ TEST ON WINDOWS
    // @@@ TEST EVERYWHERE
    // @@@ env::home_dir is deprecated, use https://crates.io/crates/dirs instead
    dotenv_err(
        &prog_name,
        DOT_ENV_FILE.to_string(),
        dotenv::from_filename(DOT_ENV_FILE),
    );
    dotenv_err(
        &prog_name,
        ENV_FILE.to_string(),
        dotenv::from_filename(ENV_FILE),
    );
    if let Some(mut p) = dirs::home_dir() {
        p.push(DOT_ENV_FILE);
        // println!("@@@ p={:?}", p.as_path());
        dotenv_err(
            &prog_name,
            p.to_string_lossy().to_string(),
            dotenv::from_path(p.as_path()),
        );
    }
    if let Some(mut p) = dirs::config_dir() {
        p.push(ENV_FILE);
        // println!("@@@ p={:?}", p.as_path());
        dotenv_err(
            &prog_name,
            p.to_string_lossy().to_string(),
            dotenv::from_path(p.as_path()),
        );
    }
    let matches = cli_for_ynab::build_clap_app().get_matches();
    if let Err(e) = cli_for_ynab::run(&prog_name, matches) {
        eprintln!("{} error: {}", prog_name, e);
        std::process::exit(1);
    }
}
