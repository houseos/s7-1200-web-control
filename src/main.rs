extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("s7-1200-web-control")
        .version("0.1")
        .author("Benjamin Schilling <benjamin.schilling33@gmail.com>")
        .arg(
            Arg::with_name("ip")
                .help("IPv4 address of S7-1200")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("key")
                .help("Key to be set")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("value")
                .help("New value for the key")
                .required(true)
                .index(3),
        )
        .get_matches();

    let target_ip = matches.value_of("ip").unwrap();
    let param_key = matches.value_of("key").unwrap();
    let param_value = matches.value_of("value").unwrap();

    println!(
        "Setting {} to {} at IP address {}",
        param_key, param_value, target_ip
    );

    // This will POST a body of `foo=bar&baz=quux`
    let login_params = [("Login", "admin"), ("Password", ""), ("Redirection", "")];
    let control_params = [(param_key, param_value)];
    let use_cookie_store = true;
    let client = reqwest::blocking::Client::builder()
        .cookie_store(use_cookie_store)
        .danger_accept_invalid_certs(true)
        .http1_title_case_headers()
        .build()?;
    let login_url = format!("https://{}/FormLogin", target_ip);
    let res = client.post(&login_url).form(&login_params).send();
    println!("{:#?}", res);
    let control_url = format!("https://{}/awp/control/control.html", target_ip);
    let res = client.post(&control_url).form(&control_params).send();
    println!("{:#?}", res);
    Ok(())
}
