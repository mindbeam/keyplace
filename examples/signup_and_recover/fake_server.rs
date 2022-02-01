use std::collections::BTreeMap;

use keyplace::{CustodialAgentKey, UserAuthKey};

pub struct KeyRecord {
    pub label: String,
    /// You should implement challenge/response rather than a bearer token mmkay?
    pub user_auth_key: UserAuthKey,
    pub custodial_key: CustodialAgentKey,
}

pub struct AuthQuery {
    pub label: String,
    pub user_auth_key: UserAuthKey,
}

pub struct AuthMatch {
    pub label: String,
    pub custodial_key: CustodialAgentKey,
}

pub struct UserRecord {
    uid: String,
    name: String,
    keys: Vec<KeyRecord>,
}

#[derive(Default)]
pub struct FakeServer {
    user_database: BTreeMap<String, UserRecord>,
}

pub struct Session {
    /// We're just pretending here. You should have a real session of some kind, or auth every transaction with an existing authkey independently
    uid: String,
}

impl FakeServer {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn register<U, T>(&mut self, uid: U, name: T, keys: Vec<KeyRecord>) -> Result<Session, &'static str>
    where
        U: ToString,
        T: ToString,
    {
        if keys.len() == 0 {
            return Err("Must provide at least one key to register");
        }
        use std::collections::btree_map::Entry::{Occupied, Vacant};
        match self.user_database.entry(uid.to_string()) {
            Vacant(e) => {
                e.insert(UserRecord {
                    uid: uid.to_string(),
                    name: name.to_string(),
                    keys,
                });
                Ok(Session { uid: uid.to_string() })
            },
            Occupied(_) => Err("User already exists"),
        }
    }

    pub fn auth(&mut self, uid: String, lak: AuthQuery) -> Result<(Session, AuthMatch), &'static str> {
        let mut user = self.user_database.get(&uid).ok_or("Beat it buddy")?;

        match user.keys.iter().find(|k| k.label == lak.label) {
            Some(key) if key.user_auth_key == lak.user_auth_key => {
                return Ok((
                    Session { uid },
                    AuthMatch {
                        label: lak.label,
                        custodial_key: key.custodial_key.clone(),
                    },
                ));
            },
            _ => Err("Beat it buddy"), // Label wasn't found OR the passphrase didn't match
        }
    }

    pub fn set_keys(&mut self, session: &Session, set_keys: Vec<KeyRecord>) -> Result<(), &'static str> {
        let user_record = self.user_database.get_mut(&session.uid).ok_or("User not found")?;
        // We're "logged in" because they gave us a "session"

        for set_key in set_keys {
            match user_record.keys.iter_mut().find(|k| k.label == set_key.label) {
                Some(key) => {
                    // overwrite the existing entry. Could alternately archive them?
                    key.user_auth_key = set_key.user_auth_key;
                    key.custodial_key = set_key.custodial_key;
                },
                _ => user_record.keys.push(set_key),
            }
        }
        Ok(())
    }

    pub fn recover(&self, uid: String, attempts: Vec<AuthQuery>) -> Result<AuthMatch, &'static str> {
        let user = self.user_database.get(&uid).ok_or("Beat it buddy")?;

        for attempt in attempts {
            match user.keys.iter().find(|k| k.label == attempt.label) {
                Some(key) if key.user_auth_key == attempt.user_auth_key => {
                    return Ok(AuthMatch {
                        label: attempt.label,
                        custodial_key: key.custodial_key.clone(),
                    });
                },
                _ => {},
            }
        }

        Err("Beat it buddy")
    }
}
