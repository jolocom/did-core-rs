use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    verification_method: Vec<VerificationMethod>,
    authentication: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationMethod {
    id: String,

    #[serde(rename = "type")]
    key_type: KeyTypes,
    controller: String,

    #[serde(flatten)]
    key: VerificationMethodProperties,
}

#[derive(Serialize, Deserialize)]
pub enum KeyTypes {
    JwsVerificationKey2020,
    EcdsaSecp256k1VerificationKey2019,
    Ed25519VerificationKey2018,
    GpgVerificationKey2020,
    RsaVerificationKey2018,
    X25519KeyAgreementKey2019,
    SchnorrSecp256k1VerificationKey2019,
    EcdsaSecp256k1RecoveryMethod2020,
}

#[derive(Serialize, Deserialize)]
pub enum VerificationMethodProperties {
    #[serde(rename = "ethereumAddress")]
    EthereumAddress(String),
    #[serde(rename = "publicKeyHex")]
    Base16(String),
    #[serde(rename = "publicKeyBase58")]
    Base58(String),
    #[serde(rename = "publicKeyBase64")]
    Base64(String),
    #[serde(rename = "publicKeyJwk")]
    Jwk(String),
    #[serde(rename = "publicKeyPem")]
    Pem(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
