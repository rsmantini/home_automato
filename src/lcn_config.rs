#[derive(Debug)]
pub struct LcnConfig {
    pub home_url: String,
    pub command_url: String,
    pub cookie: String,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse(String),
}

pub fn new(addr: &str, home: &str, command: &str, cookie: &str) -> LcnConfig {
    let home_url = String::from("http://") + addr + home;
    let command_url = String::from("http://") + addr + command;
    let cookie = String::from(cookie);
    LcnConfig {
        home_url,
        command_url,
        cookie,
    }
}

pub fn from_file(path: &str) -> Result<LcnConfig, Error> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let config_json: serde_json::Value = serde_json::from_reader(reader)?;
    let addr = get_string(&config_json, JSON_KEY_ADDR)?;
    let home = get_string(&config_json, JSON_KEY_HOME)?;
    let command = get_string(&config_json, JSON_KEY_CMD)?;
    let cookie = get_string(&config_json, JSON_KEY_COOKIE)?;
    Ok(new(&addr, &home, &command, &cookie))
}

fn get_string(json: &serde_json::Value, key: &str) -> Result<String, Error> {
    Ok(json
        .get(key)
        .ok_or_else(|| Error::Parse(key.to_string()))?
        .as_str()
        .ok_or_else(|| Error::Parse(key.to_string()))?
        .to_string())
}

const JSON_KEY_ADDR: &str = "address";
const JSON_KEY_HOME: &str = "home_path";
const JSON_KEY_CMD: &str = "command_path";
const JSON_KEY_COOKIE: &str = "cookie";

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Io(std::io::Error::from(e))
    }
}
