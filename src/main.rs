/*
s7-1200-web-control
Commandline utility
SPDX-License-Identifier: GPL-3.0-only
Copyright (C) 2020 Benjamin Schilling
*/

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
                .long("ip")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .help("Key to be set")
                .required(true)
                .long("key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("value")
                .help("New value for the key")
                .required(true)
                .long("value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .help("Username for the login")
                .required(true)
                .long("username")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .help("Password for the login")
                .required(false)
                .long("password")
                .takes_value(true),
        )
        .get_matches();

    let target_ip = matches.value_of("ip").unwrap();
    let param_key = matches.value_of("key").unwrap();
    let param_value = matches.value_of("value").unwrap();
    let param_username = matches.value_of("username").unwrap();
    let mut param_password = "";
    if matches.is_present("password") {
        param_password = matches.value_of("password").unwrap();
    } else {
        println!("No password given, using empty password.");
    }

    println!(
        "Setting {} to {} at IP address {}",
        param_key, param_value, target_ip
    );

    let login_params = [
        ("Login", param_username),
        ("Password", param_password),
        ("Redirection", ""),
    ];
    let control_params = [(param_key, param_value)];
    // Enable cookie store to cache authentication token
    let use_cookie_store = true;

    // Create HTTP Client
    let client = reqwest::blocking::Client::builder()
        .cookie_store(use_cookie_store)
        // accept invalid certificate of S7-1200 webserver
        .danger_accept_invalid_certs(true)
        //The login header is case sensitive
        .http1_title_case_headers()
        .build()?;
    // Create login URL and perform login
    let login_url = format!("https://{}/FormLogin", target_ip);
    let res = client.post(&login_url).form(&login_params).send();
    println!("{:#?}", res);

    // Create control URL
    let control_url = format!("https://{}/awp/control/control.html", target_ip);
    let res = client.post(&control_url).form(&control_params).send();
    println!("{:#?}", res);
    Ok(())
}
