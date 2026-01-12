use {
    anyhow::Result,
    speedtest_rs::{distance::EarthLocation, speedtest},
    std::{fmt::Write as _, sync::Arc},
};

const MAX_SPONSOR_LENGTH: usize = 20;
const MAX_NAME_LENGTH: usize = 20;
const TOP_X_NUM_SERVERS: usize = 10;
const DEFAULT_SERVER_COUNT: usize = 3;

/// Represents a speed test server with its metadata.
#[derive(Debug, Clone)]
pub struct Server {
    /// Unique identifier for the server.
    pub id: u32,
    /// The sponsor/provider of the server.
    pub sponsor: Arc<str>,
    /// The name of the server location.
    pub name: Arc<str>,
    /// Distance from the client in kilometers.
    pub distance_km: f32,
    /// The URL used for testing.
    pub url: String,
}

impl Server {
    /// Converts this `Server` into the `speedtest_rs` crate's server type.
    #[must_use]
    pub fn to_speedtest_server(&self) -> speedtest_rs::speedtest::SpeedTestServer {
        speedtest_rs::speedtest::SpeedTestServer {
            id: self.id,
            sponsor: self.sponsor.to_string(),
            name: self.name.to_string(),
            distance: Some(self.distance_km),
            url: self.url.clone(),
            country: String::new(),
            host: String::new(),
            location: EarthLocation::default(),
        }
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Server {} - {} ({}) - {:.1} km",
            self.id,
            ellipsize(&self.sponsor, MAX_SPONSOR_LENGTH),
            ellipsize(&self.name, MAX_NAME_LENGTH),
            self.distance_km
        )
    }
}

fn ellipsize(text: &str, max_len: usize) -> String {
    match text.len().cmp(&max_len) {
        std::cmp::Ordering::Greater => {
            format!("{}...", &text[..max_len - 3])
        }
        _ => text.to_string(),
    }
}

/// A collection of available speed test servers.
pub struct ServerList {
    /// The list of servers.
    pub servers: Vec<Server>,
}

impl ServerList {
    /// Formats the list of servers as a human-readable table string.
    #[must_use]
    pub fn format_table(&self) -> String {
        let mut output = String::new();
        output.push_str("Available Servers:\n");
        writeln!(
            output,
            "{:<10} {:<30} {:<40} {:<10}",
            "ID", "Sponsor", "Name", "Distance"
        )
        .ok();
        writeln!(output, "{}", "-".repeat(100)).ok();

        for server in &self.servers {
            writeln!(
                output,
                "{:<10} {:<30} {:<40} {:<10.2}",
                server.id, server.sponsor, server.name, server.distance_km
            )
            .ok();
        }

        output
    }

    /// # Errors
    /// Returns an error if unable to retrieve or parse the server list from the speedtest API.
    pub fn get_servers(num: usize) -> Result<Vec<Server>> {
        let config = speedtest::get_configuration()
            .map_err(|e| anyhow::anyhow!("Failed to retrieve speedtest configuration: {e:?}"))?;

        let servers = speedtest::get_server_list_with_config(&config).map_err(|e| {
            anyhow::anyhow!("Failed to retrieve server list from speedtest API: {e:?}")
        })?;

        let sorted_servers: Vec<speedtest::SpeedTestServer> =
            servers.servers_sorted_by_distance(&config);

        let result: Vec<Server> = sorted_servers
            .iter()
            .take(num)
            .map(|s| Server {
                id: s.id,
                sponsor: Arc::from(s.sponsor.as_str()),
                name: Arc::from(s.name.as_str()),
                distance_km: s.distance.unwrap_or(0.0),
                url: s.url.clone(),
            })
            .collect();

        Ok(result)
    }

    /// # Errors
    /// Will return `Err` if unable to retrieve server list.
    pub fn get_top_x(x: usize) -> Result<Self> {
        let servers = Self::get_servers(x)?;
        Ok(Self { servers })
    }

    /// # Errors
    /// Will return `Err` if unable to retrieve server list.
    pub fn list_servers() -> Result<Self> {
        Self::get_top_x(TOP_X_NUM_SERVERS)
    }

    /// # Errors
    /// Will return `Err` if server ID is invalid or not found.
    pub fn select_server(server_id: Option<String>) -> Result<Vec<Server>> {
        if let Some(id_str) = server_id {
            let id = id_str
                .parse::<u32>()
                .map_err(|_| anyhow::anyhow!("Server ID must be a valid number"))?;
            let all_servers = Self::get_servers(TOP_X_NUM_SERVERS)?;
            let filtered: Vec<Server> = all_servers.into_iter().filter(|s| s.id == id).collect();

            if filtered.is_empty() {
                anyhow::bail!("Server with ID=\"{id}\" not found in available servers.");
            }
            Ok(filtered)
        } else {
            Self::get_servers(DEFAULT_SERVER_COUNT)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsize() {
        assert_eq!(ellipsize("short", 10), "short");
        assert_eq!(ellipsize("a very long sponsor name", 10), "a very ...");
    }

    #[test]
    fn test_server_display() {
        let server: Server = Server {
            id: 1,
            sponsor: Arc::from("Sponsor Name"),
            name: Arc::from("Server Name"),
            distance_km: 100.5,
            url: "http://testserver.com".to_string(),
        };
        let display_str = server.to_string();
        assert!(display_str.contains("Server 1"));
        assert!(display_str.contains("100.5 km"));
    }

    #[test]
    fn test_select_server_invalid_id() {
        let result = ServerList::select_server(Some("not_a_number".to_string()));
        assert!(result.is_err(), "Expected error for invalid server ID");
        if let Err(err) = result {
            assert!(err.to_string().contains("valid number"));
        }
    }

    #[test]
    fn test_select_server_valid_parse() {
        let result = ServerList::select_server(Some("12345".to_string()));
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(!msg.contains("valid number"));
        }
    }

    #[test]
    fn test_server_to_speedtest_conversion() {
        let server = Server {
            id: 42,
            sponsor: Arc::from("Test Sponsor"),
            name: Arc::from("Test Name"),
            distance_km: 50.0,
            url: "http://testserver.com".to_string(),
        };
        let speedtest_server = server.to_speedtest_server();
        assert_eq!(speedtest_server.id, 42);
        assert_eq!(speedtest_server.sponsor, "Test Sponsor");
        assert_eq!(speedtest_server.name, "Test Name");
        assert_eq!(speedtest_server.distance, Some(50.0));
    }
}