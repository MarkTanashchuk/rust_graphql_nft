use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Mime(pub mime::Mime);

#[async_graphql::Object]
impl Mime {
    async fn value(&self) -> String {
        self.0.to_string()
    }
}

impl Serialize for Mime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for Mime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<mime::Mime>()
            .map_err(Error::custom)
            .map(Mime)
    }
}
