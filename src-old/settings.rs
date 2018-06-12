use chrono::Utc;
use diesel::prelude::*;
use diesel::{insert_into, update};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use super::{orm::Connection as Db, result::Result, schema::settings, security::Encryptor};

pub fn get<K: Serialize, V: DeserializeOwned>(db: &Db, enc: &Encryptor, k: &K) -> Result<V> {
    let k = serde_json::to_string(k)?;
    let (val, salt) = settings::dsl::settings
        .select((settings::dsl::value, settings::dsl::salt))
        .filter(settings::dsl::key.eq(&k))
        .first::<(Vec<u8>, Option<Vec<u8>>)>(db)?;
    let val = match salt {
        Some(salt) => enc.decrypt(&val, &salt)?,
        None => val,
    };
    let v = serde_json::from_slice(val.as_slice())?;
    Ok(v)
}

pub fn set<K: Serialize, V: Serialize>(
    db: &Db,
    enc: &Encryptor,
    k: &K,
    v: &V,
    f: bool,
) -> Result<()> {
    let k = serde_json::to_string(k)?;
    let v = serde_json::to_vec(v)?;

    let cnt: i64 = settings::dsl::settings
        .filter(settings::dsl::key.eq(&k))
        .count()
        .get_result(db)?;

    if f {
        let (val, salt) = enc.encrypt(&v);
        if cnt == 0 {
            insert_into(settings::dsl::settings)
                .values((
                    settings::dsl::key.eq(&k),
                    settings::dsl::value.eq(&val),
                    settings::dsl::salt.eq(&salt),
                    settings::dsl::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(db)?;
        } else {
            let it = settings::dsl::settings.filter(settings::dsl::key.eq(&k));
            update(it)
                .set((
                    settings::dsl::value.eq(&val),
                    settings::dsl::salt.eq(&salt),
                    settings::dsl::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(db)?;
        }
    } else {
        if cnt == 0 {
            insert_into(settings::dsl::settings)
                .values((
                    settings::dsl::key.eq(&k),
                    settings::dsl::value.eq(&v),
                    settings::dsl::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(db)?;
        } else {
            let it = settings::dsl::settings.filter(settings::dsl::key.eq(&k));
            update(it)
                .set((
                    settings::dsl::value.eq(&v),
                    settings::dsl::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(db)?;
        }
    }

    Ok(())
}