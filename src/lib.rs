//! # Keyplace
//!
//! ```
//! # use keyplace::{AgentKey,PassKey};
//! let agentkey = AgentKey::create(None);
//!
//! let passkey = PassKey::new("I like turtles");
//! let custkey = agentkey.custodial_key(passkey);
//!
//! // the custodial key is safe to send to the server
//! // Never send the passkey to anyone!!
//!
//! let passkey2 = PassKey::new("I like turtles");
//! let agentkey2 = AgentKey::from_custodial_key(custkey, passkey2).unwrap();
//! ```

extern crate zeroize;

mod error;
pub mod key_manager;
mod keys;
mod signature;
mod util;

pub use error::Error;
pub use key_manager::KeyManager;
pub use keys::{AgentId, AgentKey, CustodialAgentKey, PassKey, UserAuthKey};
pub use signature::Signature;
