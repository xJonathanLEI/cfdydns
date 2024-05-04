use std::time::Duration;

use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub struct CloudflareClient {
    http_client: reqwest::blocking::Client,
    token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DnsRecord {
    pub id: String,
    pub content: String,
    pub comment: String,
}

#[derive(Debug, Serialize)]
struct UpdateDnsRecordRequest<'a> {
    content: &'a str,
    comment: &'a str,
}

#[derive(Debug, Deserialize)]
struct Response<T> {
    result: T,
    success: bool,
}

#[derive(Debug, Deserialize)]
struct Zone {
    id: String,
}

impl CloudflareClient {
    pub fn new(api_token: String) -> Self {
        Self {
            http_client: reqwest::blocking::ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            token: api_token,
        }
    }

    pub fn get_public_ip_address(&self) -> Result<String> {
        let body = self
            .http_client
            .get("https://cloudflare.com/cdn-cgi/trace")
            .send()?
            .text()?;

        let raw_ip = body
            .lines()
            .find_map(|line| line.strip_prefix("ip="))
            .ok_or_else(|| anyhow::anyhow!("public IP address not found in Cloudflare trace"))?;

        Ok(raw_ip.to_owned())
    }

    pub fn get_zone_id(&self, name: &str) -> Result<String> {
        let result: Vec<Zone> = self.send_get_request(&format!(
            "https://api.cloudflare.com/client/v4/zones?name={}",
            name
        ))?;

        if result.len() != 1 {
            anyhow::bail!("zone not found");
        }

        Ok(result[0].id.to_owned())
    }

    pub fn get_a_record(&self, zone_id: &str, fqdn: &str) -> Result<DnsRecord> {
        let result: Vec<DnsRecord> = self.send_get_request(&format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}&type=A",
            zone_id, fqdn
        ))?;

        if result.is_empty() {
            anyhow::bail!("A record not found for {}", fqdn);
        }

        if result.len() != 1 {
            anyhow::bail!("more than 1 A record found for {}", fqdn);
        }

        Ok(result[0].clone())
    }

    pub fn update_a_record(
        &self,
        zone_id: &str,
        record_id: &str,
        content: &str,
        comment: &str,
    ) -> Result<DnsRecord> {
        let response = self
            .http_client
            .patch(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                zone_id, record_id
            ))
            .json(&UpdateDnsRecordRequest { content, comment })
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("unsuccessful status code: {}", response.status());
        }

        let response: Response<DnsRecord> = response.json()?;
        if !response.success {
            anyhow::bail!("unsuccessful Cloudflare request");
        }

        Ok(response.result)
    }

    fn send_get_request<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("unsuccessful status code: {}", response.status());
        }

        let response: Response<T> = response.json()?;
        if !response.success {
            anyhow::bail!("unsuccessful Cloudflare request");
        }

        Ok(response.result)
    }
}
