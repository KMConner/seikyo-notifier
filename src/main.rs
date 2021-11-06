mod seikyo_client;

use crate::seikyo_client::{auth, point};
use clap::{App, AppSettings, Arg, ArgMatches, ErrorKind, SubCommand};
use std::error::Error;

fn new_app() -> App<'static, 'static> {
    App::new("seikyo_notifier")
        .subcommand(
            SubCommand::with_name("token")
                .about("Log in with username and password and get access token.")
                .arg(
                    Arg::with_name("username")
                        .short("u")
                        .long("user")
                        .help("User name to log in")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .help("Password to log in")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("balance")
                .about("Retrieve the balance of the prepaid card.")
                .arg(
                    Arg::with_name("token")
                        .long("token")
                        .help("Access token fot authentication")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
}

fn token(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let user_name = args.value_of("username").ok_or(clap::Error {
        message: "user name is not specified".to_string(),
        kind: ErrorKind::MissingRequiredArgument,
        info: None,
    })?;

    let password = args.value_of("password").ok_or(clap::Error {
        message: "password is not specified".to_string(),
        kind: ErrorKind::MissingRequiredArgument,
        info: None,
    })?;
    let token = auth::get_token(user_name.to_string(), password.to_string())?;
    println!("Your access token is {}", token);
    Ok(())
}

fn balance(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let token = args.value_of("token").ok_or(clap::Error {
        message: "access token is not specified".to_string(),
        kind: ErrorKind::MissingRequiredArgument,
        info: None,
    })?;

    let token = token.to_string();
    let balance = point::get_prepaid_amount(&token)?;
    println!("Your prepaid card balance is {} yen", balance);

    Ok(())
}

fn main() {
    let app = new_app();

    let matches = app.get_matches();

    let subcommand = matches.subcommand();
    match subcommand {
        ("token", Some(m)) => token(m),
        ("balance", Some(m)) => balance(m),
        _ => panic!("invalid command !"),
    }
    .unwrap()
}
