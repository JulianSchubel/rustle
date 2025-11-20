use chrono::{DateTime, Utc};

use crate::extract;
use crate::transform;

/** -----------------------------
*  Transform
*  -----------------------------
*
*  Apply transformations to the raw record.
*
*  ∙ Parse "timestamp" field into a database-friendly type.
*  ∙ Normalize "tag" field by trimming whitespace and ensuring lowercase.
*  ∙ Filter out values with no "tag" field.
*  ∙ Derive an additional field - indicating whether "value" field is greater than zero.
*  */
pub fn transform(raw: extract::RawRecord) -> Option<transform::TransformedRecord> {
    /* Normalize tag field: trim and lowercase */
    let tag = raw.tag.trim().to_lowercase();
    /* Filter on tag field: skip records with an empty tag after trimming */
    if tag.is_empty() {
        return None;
    }

    /* Parse timestamp (ISO-8601) into unix seconds (database-friendly type) */
    let dt_res: Result<DateTime<Utc>, _> = raw.timestamp.parse();
    let ts = match dt_res {
        Ok(dt) => dt.timestamp(),
        Err(_) => {
            /* fallback: try parsing as RFC3339 via chrono parse */
            match DateTime::parse_from_rfc3339(&raw.timestamp) {
                Ok(dt) => dt.with_timezone(&Utc).timestamp(),
                /* Drop invalid timestamp */
                Err(_) => return None,
            }
        }
    };

    /* Derive additional field indicating whether value is greater than zero */
    let positive = if raw.value > 0.0 { 1 } else { 0 };
    Some(transform::TransformedRecord {
        id: raw.id,
        timestamp: ts,
        value: raw.value,
        tag,
        positive,
    })
}

