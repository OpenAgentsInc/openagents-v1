use actix::Message;
use bitcoin_hashes::{sha256, Hash};
use lazy_static::lazy_static;
use secp256k1::{schnorr, Secp256k1, VerifyOnly, XOnlyPublicKey};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::Row;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

lazy_static! {
    pub static ref SECP: Secp256k1<VerifyOnly> = Secp256k1::verification_only();
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: i32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
    #[serde(skip)]
    pub tagidx: Option<HashMap<char, HashSet<String>>>,
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Event {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let tags: JsonValue = row.try_get("tags")?;
        let tags: Vec<Vec<String>> = serde_json::from_value(tags).unwrap_or_default();

        Ok(Event {
            id: row.try_get("id")?,
            pubkey: row.try_get("pubkey")?,
            created_at: row.try_get("created_at")?,
            kind: row.try_get("kind")?,
            tags,
            content: row.try_get("content")?,
            sig: row.try_get("sig")?,
            tagidx: None,
        })
    }
}

impl Event {
    pub fn validate(&self) -> Result<(), &'static str> {
        // Validate event format and signature
        let canonical = self.to_canonical().ok_or("Could not canonicalize event")?;

        // Compute SHA256 of canonical form
        let digest: sha256::Hash = sha256::Hash::hash(canonical.as_bytes());
        let hex_digest = format!("{digest:x}");

        // Verify ID matches computed hash
        if self.id != hex_digest {
            return Err("Event ID does not match content hash");
        }

        // Verify signature
        let sig =
            schnorr::Signature::from_str(&self.sig).map_err(|_| "Invalid signature format")?;

        let msg = secp256k1::Message::from_slice(digest.as_ref())
            .map_err(|_| "Could not create message from digest")?;

        let pubkey =
            XOnlyPublicKey::from_str(&self.pubkey).map_err(|_| "Invalid public key format")?;

        SECP.verify_schnorr(&sig, &msg, &pubkey)
            .map_err(|_| "Invalid signature")?;

        Ok(())
    }

    pub fn to_canonical(&self) -> Option<String> {
        let elements = vec![
            serde_json::Value::Number(0.into()), // id placeholder
            serde_json::Value::String(self.pubkey.clone()),
            serde_json::Value::Number(self.created_at.into()),
            serde_json::Value::Number(self.kind.into()),
            self.tags_to_canonical(),
            serde_json::Value::String(self.content.clone()),
        ];

        serde_json::to_string(&serde_json::Value::Array(elements)).ok()
    }

    fn tags_to_canonical(&self) -> serde_json::Value {
        let mut tags = Vec::new();
        for tag in &self.tags {
            let tag_array = tag
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect();
            tags.push(serde_json::Value::Array(tag_array));
        }
        serde_json::Value::Array(tags)
    }

    pub fn build_index(&mut self) {
        if self.tags.is_empty() {
            return;
        }

        let mut idx: HashMap<char, HashSet<String>> = HashMap::new();

        for tag in self.tags.iter().filter(|t| t.len() > 1) {
            if let Some(tag_char) = tag.first().and_then(|s| s.chars().next()) {
                if let Some(tag_val) = tag.get(1) {
                    idx.entry(tag_char).or_default().insert(tag_val.clone());
                }
            }
        }

        self.tagidx = Some(idx);
    }

    pub fn generic_tag_val_intersect(&self, tagname: char, check: &HashSet<String>) -> bool {
        match &self.tagidx {
            Some(idx) => match idx.get(&tagname) {
                Some(valset) => !valset.is_disjoint(check),
                None => false,
            },
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCmd {
    pub cmd: String,
    pub event: Event,
}