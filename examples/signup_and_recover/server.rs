use std::collections::BTreeMap;

use keyplace::{CustodialAgentKey, UserAuthKey};

pub struct ServerKey {
    label: String,
    /// You should implement challenge/response rather than a bearer token mmkay?
    cust_authkey: UserAuthKey,
    cust_agentkey: CustodialAgentKey,
}

pub struct AuthQuery {
    label: String,
    auth_key: UserAuthKey,
}

pub struct AuthMatch {
    label: String,
    agent_key: CustodialAgentKey,
}

pub struct User {
    uid: String,
    name: String,
    keys: Vec<ServerKey>,
}

#[derive(Default)]
pub struct Server {
    users: BTreeMap<String, User>,
}

pub struct Session {
    /// We're just pretending here. You should have a real session of some kind, or auth every transaction with an existing authkey independently
    uid: String,
}

impl Server {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn register(
        &mut self,
        uid: String,
        name: String,
        keys: Vec<ServerKey>,
    ) -> Result<Session, &'static str> {
        if keys.len() == 0 {
            return Err("Must provide at least one key to register");
        }
        use std::collections::btree_map::Entry::{Occupied, Vacant};
        match self.users.entry(uid) {
            Vacant(e) => {
                e.insert(User { uid, name, keys });
                Ok(Session { uid })
            }
            Occupied(_) => Err("User already exists"),
        }
    }

    pub fn auth(
        &mut self,
        uid: String,
        lak: AuthQuery,
    ) -> Result<(Session, AuthMatch), &'static str> {
        let mut user = self.users.get(&uid).ok_or("Beat it buddy")?;

        match user.keys.iter().find(|k| k.label == lak.label) {
            Some(key) if key.cust_authkey == lak.auth_key => {
                return Ok((
                    Session { uid },
                    AuthMatch {
                        label: lak.label,
                        agent_key: key.cust_agentkey,
                    },
                ));
            }
            _ => Err("Beat it buddy"), // Label wasn't found OR the passphrase didn't match
        }
    }

    fn set_keys(
        &mut self,
        session: &Session,
        set_keys: Vec<ServerKey>,
    ) -> Result<(), &'static str> {
        let mut user = self.users.get(&session.uid).ok_or("User not found")?;
        // We're "logged in" because they gave us a "session"

        for set_key in set_keys {
            match user.keys.iter().find(|k| k.label == set_key.label) {
                Some(key) => {
                    // overwrite the existing entry. Could alternately archive them?
                    key.cust_authkey = set_key.cust_authkey;
                    key.cust_agentkey = set_key.cust_agentkey;
                }
                _ => user.keys.push(set_key),
            }
        }
        Ok(())
    }

    fn recover(&self, uid: String, attempts: Vec<AuthQuery>) -> Result<AuthMatch, &'static str> {
        let mut user = self.users.get(&uid).ok_or("Beat it buddy")?;

        for attempt in attempts {
            match user.keys.iter().find(|k| k.label == attempt.label) {
                Some(key) if key.cust_authkey == attempt.auth_key => {
                    return Ok(AuthMatch {
                        label: attempt.label,
                        agent_key: key.cust_agentkey,
                    });
                }
                _ => {}
            }
        }

        Err("Beat it buddy")
    }
}
