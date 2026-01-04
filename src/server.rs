use {anyhow::Result, speedtest_rs::speedtest};

const MAX_SPONSOR_LENGTH: usize = 20;
const MAX_NAME_LENGTH: usize = 20;
const TOP_X_NUM_SERVERS: usize = 10;

#[derive(Debug, Clone)]
pub struct Server {
    pub id: u32,
    pub sponsor: String,
    pub name: String,
    pub distance_km: f32,
}

impl Server {
    /// Convert a Server to a SpeedTestServer with required fields
    pub fn to_speedtest_server(&self) -> speedtest_rs::speedtest::SpeedTestServer {
        speedtest_rs::speedtest::SpeedTestServer {
            id: self.id,
            sponsor: self.sponsor.clone(),
            name: self.name.clone(),
            distance: Some(self.distance_km),
            url: String::new(),
            country: String::new(),
            host: String::new(),
            location: Default::default(),
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
        std::cmp::Ordering::Greater => format!("{}...", &text[..max_len - 3]),
        _ => text.to_string(),
    }
}

pub struct ServerList {
    pub servers: Vec<Server>,
}

impl ServerList {
    pub fn get_top_x(x: usize) -> Result<Self> {
        let servers = ServerList::get_servers(x)?;
        Ok(ServerList { servers })
    }

    pub fn get_servers(num: usize) -> Result<Vec<Server>> {
        let config = speedtest::get_configuration().map_err(|e| anyhow::anyhow!("Failed to get config: {:?}", e))?;

        let servers = speedtest::get_server_list_with_config(&config)
            .map_err(|e| anyhow::anyhow!("Failed to get server list: {:?}", e))?;

        let sorted_servers: Vec<speedtest::SpeedTestServer> = servers.servers_sorted_by_distance(&config);

        let result: Vec<Server> = sorted_servers
            .iter()
            .take(num)
            .map(|s: &speedtest::SpeedTestServer| Server {
                id: s.id,
                sponsor: s.sponsor.clone(),
                name: s.name.clone(),
                distance_km: s.distance.unwrap_or(0.0),
            })
            .collect();

        Ok(result)
    }

    pub fn list_servers() -> Result<Self> {
        Self::get_top_x(TOP_X_NUM_SERVERS).map(|sl: ServerList| sl)
    }

    pub fn select_server(server_id: Option<String>) -> Result<Vec<Server>> {
        if let Some(id_str) = server_id {
            let id: u32 = id_str
                .parse::<u32>()
                .map_err(|_| anyhow::anyhow!("Server ID must be a valid number"))?;
            let all_servers = ServerList::get_servers(10)?;
            let filtered: Vec<Server> = all_servers.into_iter().filter(|s| s.id == id).collect();

            if filtered.is_empty() {
                anyhow::bail!("Server with ID {} not found", id);
            }
            Ok(filtered)
        } else {
            ServerList::get_servers(3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsize() {
        assert_eq!(ellipsize("short", 10), "short");
        // "a very long" has length 11, max_len=10, so we take first 7 chars "a very " + "..."
        assert_eq!(ellipsize("a very long sponsor name", 10), "a very ...");
    }

    #[test]
    fn test_server_display() {
        let server: Server = Server {
            id: 1,
            sponsor: "Sponsor Name".to_string(),
            name: "Server Name".to_string(),
            distance_km: 100.5,
        };
        let display_str = server.to_string();
        assert!(display_str.contains("Server 1"));
        assert!(display_str.contains("100.5 km"));
    }
}
