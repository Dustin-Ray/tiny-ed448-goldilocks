use base64::Engine;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use core::fmt;

use super::field_element::FieldElement;

impl Serialize for FieldElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes();
        let base64_str = general_purpose::STANDARD.encode(bytes);
        serializer.serialize_str(&base64_str)
    }
}

impl<'de> Deserialize<'de> for FieldElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FieldElementVisitor)
    }
}
use base64::engine::general_purpose;

struct FieldElementVisitor;

impl<'de> Visitor<'de> for FieldElementVisitor {
    type Value = FieldElement;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64-encoded string representing a FieldElement")
    }

    fn visit_str<E>(self, v: &str) -> Result<FieldElement, E>
    where
        E: de::Error,
    {
        let bytes = general_purpose::STANDARD
            .decode(v)
            .map_err(de::Error::custom)?;
        let arr: [u8; 56] = bytes
            .clone()
            .try_into()
            .map_err(|_| de::Error::invalid_length(bytes.len(), &self))?;
        Ok(FieldElement::from_bytes(&arr))
    }
}
