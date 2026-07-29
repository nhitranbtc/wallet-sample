#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Connectivity,
    RateLimited,
    ChainState,
    Unknown,
}

pub fn classify(message: &str) -> Outcome {
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(value) => value
            .get("error")
            .and_then(|error| error.get("code"))
            .and_then(serde_json::Value::as_i64)
            .map_or(Outcome::Unknown, classify_code),
        Err(_) => classify_plain_text(message),
    }
}

fn classify_code(code: i64) -> Outcome {
    match code {
        -32099..=-32000 => Outcome::ChainState,
        -32603 => Outcome::Connectivity,
        _ => Outcome::Unknown,
    }
}

fn classify_plain_text(message: &str) -> Outcome {
    const MAX_FALLBACK_BYTES: usize = 1_024;

    if message.len() > MAX_FALLBACK_BYTES {
        return Outcome::Unknown;
    }

    let normalized = message.to_ascii_lowercase();
    if normalized.contains("content-type: text/html") || normalized.contains("<html") {
        return Outcome::Unknown;
    }

    if normalized.contains("429") || normalized.contains("rate limit") {
        Outcome::RateLimited
    } else if normalized.contains("nonce") || normalized.contains("expired") {
        Outcome::ChainState
    } else if normalized.contains("timeout") || normalized.contains("connection") {
        Outcome::Connectivity
    } else {
        Outcome::Unknown
    }
}
