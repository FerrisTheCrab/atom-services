use std::collections::BTreeMap;

use mongodb::bson::{doc, Bson, Document};

use crate::ServiceInstance;

macro_rules! bad_key {
    () => {
        mongodb::error::Error::custom("bad key".to_string())
    };
}

macro_rules! cond_bad_key {
    ($x: expr) => {
        if $x == "_id" || $x.is_empty() {
            return Err(bad_key!());
        }
    };
}

macro_rules! not_found {
    () => {
        mongodb::error::Error::custom("service not found".to_string())
    };
}

pub struct Service;

impl Service {
    fn encode(s: &str) -> String {
        let mut out = String::new();

        for c in s.chars() {
            match c {
                '$' => out.push_str("$d"),
                '.' => out.push_str("$p"),
                c => out.push(c),
            }
        }

        out
    }

    fn decode(s: &str) -> String {
        let mut out = String::new();
        let mut op = false;

        for c in s.chars() {
            match c {
                '$' => op = true,
                c if op => {
                    match c {
                        'd' => out.push('$'),
                        'p' => out.push('.'),
                        _ => panic!("unknown escape sequence ${c}"),
                    };
                    op = false
                }
                c => out.push(c),
            }
        }

        out
    }

    async fn register_int(
        instance: &ServiceInstance,
        name: &str,
    ) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "_id": name };

        if instance.services.find_one(filter.clone()).await?.is_none() {
            instance.services.insert_one(filter).await?;
        }

        Ok(())
    }

    async fn deregister_int(
        instance: &ServiceInstance,
        name: &str,
    ) -> Result<(), mongodb::error::Error> {
        instance.services.delete_one(doc! { "_id": name }).await?;
        Ok(())
    }

    async fn set_int(
        instance: &ServiceInstance,
        name: &str,
        entries: Vec<(String, String)>,
    ) -> Result<bool, mongodb::error::Error> {
        let mut m_set = Document::new();
        let mut m_unset = Document::new();

        for (k, v) in entries.into_iter() {
            cond_bad_key!(k);
            if v.is_empty() {
                m_unset.insert(Self::encode(&k), Bson::String(String::new()));
            } else {
                m_set.insert(Self::encode(&k), Bson::String(v));
            }
        }

        Ok(instance
            .services
            .update_one(
                doc! { "_id": name },
                doc! { "$set": m_set.clone(), "$unset": m_unset},
            )
            .await?
            .matched_count
            == 1)
    }

    async fn show_int(
        instance: &ServiceInstance,
        id: &str,
        entries: Vec<String>,
    ) -> Result<Option<BTreeMap<String, String>>, mongodb::error::Error> {
        let mut projection = Document::new();

        for k in entries.into_iter() {
            cond_bad_key!(k);
            projection.insert(Self::encode(&k), String::new());
        }

        let mut entry = match instance
            .services
            .find_one(doc! { "_id": id})
            .projection(projection)
            .await?
        {
            Some(entry) => entry,
            None => return Ok(None),
        };

        entry.remove("_id");

        let mut out = BTreeMap::new();

        for (k, v) in entry.into_iter() {
            out.insert(Self::decode(&k), v.as_str().unwrap().to_string());
        }

        Ok(Some(out))
    }

    async fn exists_int(
        instance: &ServiceInstance,
        id: &str,
    ) -> Result<bool, mongodb::error::Error> {
        Ok(instance
            .services
            .find_one(doc! {"_id": id })
            .await?
            .is_some())
    }
}

impl Service {
    pub async fn register(
        instance: &ServiceInstance,
        name: &str,
    ) -> Result<(), mongodb::error::Error> {
        Self::register_int(instance, name).await
    }

    pub async fn deregister(
        instance: &ServiceInstance,
        name: &str,
    ) -> Result<(), mongodb::error::Error> {
        Self::deregister_int(instance, name).await
    }

    pub async fn show(
        instance: &ServiceInstance,
        id: &str,
        entries: Vec<String>,
    ) -> Result<BTreeMap<String, String>, mongodb::error::Error> {
        Ok(Self::show_int(instance, id, entries)
            .await?
            .unwrap_or_default())
    }

    pub async fn set(
        instance: &ServiceInstance,
        id: &str,
        entries: Vec<(String, String)>,
    ) -> Result<(), mongodb::error::Error> {
        if !Self::set_int(instance, id, entries).await? {
            Err(not_found!())
        } else {
            Ok(())
        }
    }

    pub async fn exists(
        instance: &ServiceInstance,
        id: &str,
    ) -> Result<bool, mongodb::error::Error> {
        Self::exists_int(instance, id).await
    }
}
