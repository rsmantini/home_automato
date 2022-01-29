use reqwest::{blocking::Client, cookie::CookieStore, Url};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, io, io::Write};

#[derive(Debug)]
pub struct LcnClient {
    pub http_client: Client,
    pub home_url: String,
    pub command_url: String,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Http(reqwest::Error),
    Auth,
}

pub fn build_lcn_client() -> Result<LcnClient, Error> {
    println!("building lcn client:");
    let jar = std::sync::Arc::new(reqwest::cookie::Jar::default());
    let urls;

    let auto_login = load_lcn_auth();
    let mut manual_login: Option<LcnLogin> = None;
    if let Ok(a) = auto_login {
        println!(" >> cached login info found");
        urls = get_urls(&a.addr, &a.proj);
        jar.as_ref()
            .add_cookie_str(&a.cookie, &urls.base.parse::<Url>().unwrap());
    } else {
        println!(" >> starting manual login");
        let l = get_login_info()?;
        urls = get_urls(&l.addr, &l.proj);
        manual_login = Some(l);
    }
    let url = urls.base.parse::<Url>().unwrap();
    jar.as_ref()
        .add_cookie_str("AspxAutoDetectCookieSupport=1", &url);
    jar.as_ref()
        .add_cookie_str("ASP.NET_SessionId=j0rurz1zivixv0fklxq3gmld", &url);

    let http_client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .cookie_provider(jar.clone())
        .build()
        .expect("could not init http client");

    if let Some(l) = manual_login {
        let payload = get_login_payload(l.uname, l.passwd);
        println!(" >> trying to authenticate in lcn app");
        http_client.post(urls.login).form(&payload).send()?;
        let cookies = jar
            .as_ref()
            .cookies(&urls.base.parse::<Url>().unwrap())
            .ok_or(Error::Auth)?;
        let cookies = cookies.to_str().map_err(|_| Error::Auth)?;
        let auth_cookie = get_auth_cookie(cookies).ok_or(Error::Auth)?;
        println!(" >> authentication succeeded... saving auth info");
        save_auth(l.addr, l.proj, auth_cookie)?;
    }

    Ok(LcnClient {
        http_client,
        home_url: urls.home,
        command_url: urls.command,
    })
}

fn load_lcn_auth() -> Result<LcnAuth, Error> {
    let fd = std::fs::File::open("lcn_auth")?;
    let reader = std::io::BufReader::new(fd);
    Ok(serde_json::from_reader(reader)?)
}

fn get_urls(addr: &str, proj: &str) -> Urls {
    let base = format!("http://{}", addr);
    let login = format!("http://{}//LCNGVS/visual.aspx", addr);
    let home = format!("http://{}/lcngvs/control.aspx?ui=Mobil&proj={}", addr, proj);
    let command = format!("http://{}/lcngvs/renderer3.aspx/AjaxButtonClicked", addr);
    Urls {
        base,
        login,
        home,
        command,
    }
}

fn get_login_info() -> Result<LcnLogin, io::Error> {
    let mut login = LcnLogin {
        addr: String::new(),
        proj: String::new(),
        uname: String::new(),
        passwd: String::new(),
    };
    print!("ip address: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut login.addr)?;
    print!("user name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut login.uname)?;
    print!("password: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut login.passwd)?;
    print!("project: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut login.proj)?;
    login.addr.truncate(login.addr.trim_end().len());
    login.uname.truncate(login.uname.trim_end().len());
    login.passwd.truncate(login.passwd.trim_end().len());
    login.proj.truncate(login.proj.trim_end().len());
    if login.proj.is_empty() {
        login.proj = get_project_from_uname(&login.uname);
    }
    Ok(login)
}

fn get_project_from_uname(uname: &str) -> String {
    uname.replace("-", " ").replace("/", ".")
}

fn get_auth_cookie(cookies: &str) -> Option<String> {
    cookies
        .split("; ")
        .find(|x| x.contains("LCN-GVS-Auth"))
        .map(|x| x.to_owned())
}

fn save_auth(addr: String, proj: String, cookie: String) -> Result<(), Error> {
    let mut fd = std::fs::File::create("lcn_auth")?;
    let x = LcnAuth { addr, proj, cookie };
    fd.write_all(serde_json::to_string(&x)?.as_bytes())?;
    Ok(())
}

fn get_login_payload(user_name: String, password: String) -> Vec<(String, String)> {
    let form: Vec<(String, String)> = vec![
        ("__LASTFOCUS".to_owned(), "".to_owned()),
        ("__EVENTTARGET".to_owned(), "".to_owned()),
        ("__EVENTARGUMENT".to_owned(), "".to_owned()),
        ("__VIEWSTATE".to_owned(), "HTCBTEyjFPDrfK2UzAUIC1ExrzGGqi1qcKNY4JqXqfHGyt2E9EZPboWkLvEGv1roo03CtrXZWYz19Jzw5oW9d3B2lbgzeJCj4Oe3NlFwqFIwpQ6hInOv94eSjLfnoR2RSie3JvKHfOgOMDPOgdNqrUfjCUtRvWaacpLcvXlCDu1coapJMhyUW+xeuig+2pGWEAEeuTB+tLFGMf+eVLrFVeAEGnv+4HPfEwFtRV3HL30QJRPkm/CeGp9ysHq6hBBespag/+JI0Y7M9U+UCf/BOZjQtYTw7QLocScQkEb4tiQCD7EI4TuKqD140g2PoIdL1MTWYBjWZhaYwem0BUK/y18TUQPaSX5sx6Cgk7wPYClntGyZ4Kfh9fT1TOREUaME/XW0sbmq19fOww+QLjOBR2Ov3RGygOqAaHnQq7+0vueKCK7ukb4X2XFwUqMBEWguzI833/3ruZM+x/giJKPJ8fNKzhI+q2smWHSh9NV56gPIyrztAyxLXDUu7n+fDzAztw5H+jlaCnxiTQlPNayXWlocuevHG+TWD8NlCmQGIj2j+DsdsNQLwOtTD/N8xOjYTCg8+A==".to_owned()),
        ("__VIEWSTATEGENERATOR".to_owned(), "4E5E161E".to_owned()),
        ("__EVENTVALIDATION".to_owned(), "2U9GlFW/H7C7A1WIYFY6UbCt0U+zNBytu2lITF1Dd/HOOOliLocITad04fcRTQIU/4cR7309pI8uwtFZDCqqSlz2+yC0f1F16ktkX56VoqZsA0PFldN8gIN+U3U1kKwLfAULG7z1jS/lyJFV5IawFmmrqXCutjBEAV3Q+3+/jEazHU7s20mcOPlcvKKGwtLb6UR6Uujxr4i1fkvQbiP9l5b81Dw=".to_owned()),
        ("loginView$UserName".to_owned(), user_name),
        ("loginView$Password".to_owned(), password),
        ("loginView$LoginButton".to_owned(), "Log in".to_owned()),
    ];
    form
}

struct LcnLogin {
    addr: String,
    proj: String,
    uname: String,
    passwd: String,
}

#[derive(Deserialize, Serialize)]
struct LcnAuth {
    addr: String,
    proj: String,
    cookie: String,
}

#[derive(Debug)]
struct Urls {
    base: String,
    login: String,
    home: String,
    command: String,
}

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

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => f.write_str(&e.to_string()),
            Error::Http(e) => f.write_str(&e.to_string()),
            Error::Auth => {
                f.write_str("Authentication error. User name or password might be wrong.")
            }
        }
    }
}
