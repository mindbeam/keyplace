use keyplace::{AgentKey, PassKey};
use server::{Server, ServerKey};

mod client;
mod server;

fn main() {
    let mut server = Server::new();
    user_signup(&mut server);
}

fn user_signup(server: &mut Server) {
    let agentkey = AgentKey::create(None);

    let passkey = PassKey::new("I am literally a cat");
    let custkey = agentkey.custodial_key(passkey);

    let auth_key = passkey.c

    server.register(
        "bob@cats.org",
        "Bob the cat",
        vec![ServerKey {
            label: "primary".to_string(),
            cust_agentkey: "woof",
        }],
    )
}

fn whoops_i_forgot_my_password() {}
