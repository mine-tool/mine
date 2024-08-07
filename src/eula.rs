use std::io::Write;
use chrono::prelude::*;
use chrono_tz::Europe::Berlin;

pub fn generate_eula() -> std::io::Result<()> {
    let mut file = std::fs::File::create("eula.txt")?;
    let now = Utc::now().with_timezone(&Berlin);
    let formatted_date = now.format("%a %b %d %H:%M:%S %Z %Y").to_string();
    let content = format!("#By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).\n#{date}\neula=true", date=formatted_date);

    file.write_all(content.as_bytes())?;
    Ok(())
}
