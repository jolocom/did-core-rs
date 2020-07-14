//! # DID-Core
//!
//! A Library for the core data model of the W3C Decentralized Identifier Specification

use serde::{Deserialize, Serialize};

/// DID Document
///
/// A set of data describing the DID Subject, as defined by the [DID-Core Spec](https://www.w3.org/TR/did-core/#dfn-did-documents).
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    verification_method: Vec<VerificationMethod>,
    authentication: Option<Vec<String>>,
}

/// Verification Method Section
///
/// Set of parameters that can be used together with a process or protocol to independantly verify a proof, as defined by the [DID-Core Spec](https://www.w3.org/TR/did-core/#dfn-verification-method)
#[derive(Serialize, Deserialize)]
pub struct VerificationMethod {
    id: String,

    #[serde(flatten)]
    key_type: VerificationMethodType,
    controller: String,

    #[serde(flatten)]
    key: VerificationMethodProperties,
}

/// Verification Method Types
///
/// As defined by the [DID-Core Registries §4.5](https://w3c.github.io/did-spec-registries/#verification-method-types)
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VerificationMethodType {
    JwsVerificationKey2020,
    EcdsaSecp256k1VerificationKey2019,
    Ed25519VerificationKey2018,
    GpgVerificationKey2020,
    RsaVerificationKey2018,
    X25519KeyAgreementKey2019,
    SchnorrSecp256k1VerificationKey2019,
    EcdsaSecp256k1RecoveryMethod2020,
}

/// Verification Method Properties
///
/// As defined by the [DID-Core Registries §4.4](https://w3c.github.io/did-spec-registries/#verification-method-properties)
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
