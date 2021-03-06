use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::fmt;

use crate::{util::AsBytes, AgentKey, Error};

#[derive(Serialize, Deserialize)]
pub struct Signature(
    #[serde(
        serialize_with = "crate::util::ser_as_base64",
        deserialize_with = "crate::util::de_from_base64"
    )]
    pub(crate) [u8; 64],
);

impl Signature {
    pub fn new<T>(agentkey: &AgentKey, content: T) -> Result<Self, Error>
    where
        T: HashHelper,
    {
        let mut hasher: Sha512 = Sha512::default();
        content.hash(&mut hasher);

        let sig = agentkey
            .keypair
            .sign_prehashed(hasher, Some(b"allegation"))
            .unwrap();

        Ok(Signature(sig.to_bytes()))
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0[..], STANDARD_NO_PAD))
    }
}
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArtifactId:{}", base64::encode(&self.0[..]))
    }
}

pub trait HashHelper {
    fn hash(&self, hasher: &mut Sha512);
}

impl<A> HashHelper for (A,)
where
    A: AsBytes,
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.update(self.0.as_bytes());
    }
}
impl<A, B> HashHelper for (A, B)
where
    A: AsBytes,
    B: AsBytes,
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
    }
}
impl<A, B, C> HashHelper for (A, B, C)
where
    A: AsBytes,
    B: AsBytes,
    C: AsBytes,
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
        hasher.update(self.2.as_bytes());
    }
}
impl<A, B, C, D> HashHelper for (A, B, C, D)
where
    A: AsBytes,
    B: AsBytes,
    C: AsBytes,
    D: AsBytes,
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.update(self.0.as_bytes());
        hasher.update(self.1.as_bytes());
        hasher.update(self.2.as_bytes());
        hasher.update(self.3.as_bytes());
    }
}
