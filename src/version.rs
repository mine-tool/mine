use std::cmp::Ordering;

#[derive(Debug)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: Option<u32>,
}

impl Version {
    pub fn from_str(version: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 2 || parts.len() > 3 {
            return Err("Invalid version format".to_string());
        }

        let major = parts[0].parse::<u32>().map_err(|_| "Invalid major version".to_string())?;
        let minor = parts[1].parse::<u32>().map_err(|_| "Invalid minor version".to_string())?;
        let patch = if parts.len() == 3 {
            Some(parts[2].parse::<u32>().map_err(|_| "Invalid patch version".to_string())?)
        } else {
            None
        };

        Ok(Version { major, minor, patch })
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major != other.major {
            return Some(self.major.cmp(&other.major));
        }

        if self.minor != other.minor {
            return Some(self.minor.cmp(&other.minor));
        }

        match (self.patch, other.patch) {
            (Some(p1), Some(p2)) => Some(p1.cmp(&p2)),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => Some(Ordering::Equal),
        }
    }
}
