use std::fmt;
use std::time::Duration;

#[non_exhaustive]
enum Gateways {
    GatewayIpfsIo,
    IpfsIo,
    Cloudflare,
    Cf,
}

impl fmt::Display for Gateways {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = match self {
            Self::GatewayIpfsIo => "https://gateway.ipfs.io/ipfs/",
            Self::IpfsIo => "https://ipfs.io/ipfs/",
            Self::Cloudflare => "https://cloudflare-ipfs.com/ipfs/",
            Self::Cf => "https://cf-ipfs.com/ipfs/",
        };

        write!(f, "{url}")
    }
}

impl Gateways {
    fn get_all() -> Vec<Self> {
        vec![
            Self::GatewayIpfsIo,
            Self::IpfsIo,
            Self::Cloudflare,
            Self::Cf,
        ]
    }
}

/// Converts an IPFS URL to a regular HTTP URL through a pre-defined provider
pub async fn normalize_ipfs_url(link: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let timeout = Duration::from_secs(15);

    for gateway in Gateways::get_all() {
        let url = link.replace("ipfs://", &gateway.to_string());

        if client.get(&url).timeout(timeout).send().await.is_ok() {
            return Ok(url);
        }
    }

    Err("Cannot process IPFS link".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_normalize_ipfs_url() {
        let url = "ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq".to_string();
        let normalized_url = normalize_ipfs_url(url).await.expect("Valid IPFS URL");

        assert!(
           normalized_url == "https://gateway.ipfs.io/ipfs/bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq" ||
           normalized_url == "https://ipfs.io/ipfs/bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq" ||
           normalized_url == "https://cloudflare-ipfs.com/ipfs/bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq" ||
           normalized_url == "https://cf-ipfs.com/ipfs/bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq"
        );
    }
}
