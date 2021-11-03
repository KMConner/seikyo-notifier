use seikyo_notifier::seikyo_client::auth;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} [user id] [password]", args[0]);
        return;
    }
    let user_id: String = args[1].clone();
    let password: String = args[2].clone();
    let result = auth::get_token(user_id, password);
    match result {
        Ok(t) => println!("Your token is {}", t),
        Err(e) => panic!("failed to get token: {}", e)
    }
}
