use core::panic;
use std::slice::Windows;

use fake_server::{FakeServer, KeyRecord, Session};
use keyplace::{AgentKey, CustodialAgentKey, Error, PassKey};

use crate::fake_server::AuthQuery;

mod client;
mod fake_server;

fn main() {
    let mut server = FakeServer::new();

    let passkey = PassKey::new("correct horse battery staple");

    let agentkey1 = user_signup(&mut server, &passkey).expect("user signup");
    // Typically you'd drop this as soon as you're done using it
    // drop(agentkey1); // Don't keep this around for too long y'all

    tequila_tasting_party(passkey);

    let agentkey2 = whoops_i_forgot_my_password(&server).expect("password recovery");

    assert_eq!(agentkey1, agentkey2);
    println!("Huzzah!")

    // TODO demo encryption/decryption using the AgentKey
}

fn user_signup(server: &mut FakeServer, passkey: &PassKey) -> Result<AgentKey, &'static str> {
    // AgentKey is basically just the ECC Keypair
    let agentkey = AgentKey::create(None);

    let custodial_key = agentkey.custodial_key(&passkey);

    // NEVER send the passkey to the server! This is for your eyes only!
    let user_auth_key = passkey.auth();
    // UserAuthKey is a salted version, which is safe-ISH to send, but it's still a bearer token,
    // which is bad because it can be replayed. The replayer would still not be able to XOR your
    // agentkey back out from the custodial key, because you've not disclosed your passkey...
    // but it's all about defense-in-depth here folks.

    let session = server.register(
        "bob@cats.org",
        "Bob the cat",
        vec![KeyRecord {
            label: "primary".to_string(),
            custodial_key,
            user_auth_key,
        }],
    )?;

    let recovery_questions = get_all_recovery_questions();
    assert_eq!(recovery_questions.len(), 8);

    // Could do arbitrary combinations, not just windows.
    // Remember, we have to store every variation we might need later for recovery.
    // Note that the combinatorial nature of this is somewhat in tension with the performance
    // of Scrypt below, which is why we're doing windows rather than all possible combinations of 4

    // Each Passkey is generated by Scrypt hashing the Passphrase. This makes it difficult to generate rainbow tables.
    // Given the number of recovery passkeys, you may consider backgrounding this process for the user, as it could take
    // a minute to get through all of them.
    println!("Generating recovery question key records...");
    let recovery_question_key_records: Vec<KeyRecord> = recovery_questions
        .windows(4)
        .enumerate()
        .map(|(i, win)| {
            let passkey = generate_passkey_from_questions(&win);

            KeyRecord {
                label: format!("rq_combo_{}", i).to_string(),
                user_auth_key: passkey.auth(),
                custodial_key: agentkey.custodial_key(&passkey),
            }
        })
        .collect();
    println!("Done. Generated {} key records", recovery_question_key_records.len());

    assert_eq!(recovery_question_key_records.len(), 5);

    let printed_recovery_codes = vec![
        "Y62X ETZZ V5N6 UGA8 9F9D 5APH YKN7 A6A4",
        "6NJP 8NH6 362Z 7XPZ B32K 37ZE YBNW RCWY",
        "CHRN 4FS3 7TUU JEKU ZEN8 Z6PY DAQT ZYFA",
        // And so on...
    ];
    // Ideally you'd add a recovery question or two to each printed recovery key to make it harder
    // for a thief to gain access using the printed recovery sheet alone

    println!("Generating printed recovery code key records...");
    let key_records_for_printed: Vec<KeyRecord> = printed_recovery_codes
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let passkey = PassKey::new(k);
            KeyRecord {
                label: format!("printed_code_{}", i).to_string(), // This is a little goofy. Could omit the label, or do a prefix search or whatever
                user_auth_key: passkey.auth(),
                custodial_key: agentkey.custodial_key(&passkey),
            }
        })
        .collect();
    println!("Done. Generated {} key_records", key_records_for_printed.len());

    let mut all_key_records = recovery_question_key_records;
    all_key_records.extend(key_records_for_printed);

    server.set_keys(&session, all_key_records)?;

    Ok(agentkey)
}

fn generate_passkey_from_questions(slice: &[RecoveryQuestion]) -> PassKey {
    // There's a better way than string concatenation, but this illustrates the point

    let mut passphrase = String::new();
    for q in slice {
        passphrase.push_str(&q.question.to_lowercase());
        passphrase.push_str("|");
        passphrase.push_str(&q.question.to_lowercase());
        passphrase.push_str(";");
    }

    PassKey::new(&passphrase)
}
struct RecoveryQuestion {
    question: &'static str,
    answer: &'static str,
}

fn get_all_recovery_questions() -> Vec<RecoveryQuestion> {
    use RecoveryQuestion as RQ;

    vec![
        RQ {
            question: "What is your DOB",
            answer: "08/27/2018",
        },
        RQ {
            question: "How many were in your litter, total?",
            answer: "8",
        },
        RQ {
            question: "Where did you get adopted from?",
            answer: "Humane society",
        },
        RQ {
            question: "What is your favorite thing in the world?",
            answer: "Eating papa's toe socks",
        },
        RQ {
            question: "What was the name of your first love?",
            answer: "Nom noms",
        },
        RQ {
            question: "What is your favorite place to sleep during the day?",
            answer: "The blue chair",
        },
        RQ {
            question: "What is your favorite time to meow randomly?",
            answer: "5:00am",
        },
        RQ {
            question: "What do you like to do around mealtime?",
            answer: "Bite papa's knees",
        },
    ]
}

fn tequila_tasting_party(passkey: PassKey) {
    // One tequila
    // Two tequila
    // Three tequila
    drop(passkey);
}

fn whoops_i_forgot_my_password(server: &FakeServer) -> Result<AgentKey, &'static str> {
    let mut recovery_questions = get_all_recovery_questions();

    // Lets mess some of them up
    recovery_questions[0].answer = "04/20/1969";
    recovery_questions[1].answer = "900";
    recovery_questions[2].answer = "spontaneously pukes on carpet";
    recovery_questions[7].answer = "wait patiently";

    assert_eq!(recovery_questions.len(), 8);

    // Just doing recovery questions for now

    println!("Generating recovery question passkeys...");
    let recovery_question_passkeys: Vec<PassKey> = recovery_questions
        .windows(4)
        .map(|win| generate_passkey_from_questions(&win))
        .collect();

    println!("Done. Generated {} passkeys", recovery_question_passkeys.len());

    println!("Generating AuthQueries");
    let attempts: Vec<AuthQuery> = recovery_question_passkeys
        .iter()
        .enumerate()
        .map(|(i, passkey)| AuthQuery {
            label: format!("rq_combo_{}", i).to_string(), // this is a little goofy
            user_auth_key: passkey.auth(),
        }) // XOR the agentkey with each passkey
        .collect();

    let auth_match = server.recover("bob@cats.org".to_string(), attempts)?;

    let agentkey = recovery_question_passkeys.iter().find_map(|passkey| {
        match AgentKey::from_custodial_key(auth_match.custodial_key.clone(), passkey) {
            Ok(agentkey) => Some(agentkey),
            Err(Error::Mac(_)) => None,
            _ => panic!("Illegal error type. TODO: handle this better"),
        }
    });

    agentkey.ok_or("Server returned an auth match, but its custodial key failed to match all passkeys")
}
