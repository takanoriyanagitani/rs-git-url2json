use std::io;

use io::BufWriter;
use io::Write;

use io::BufRead;

use git_url_parse::GitUrl;

use git_url_parse::types::provider::generic::GenericProvider;
use git_url_parse::types::provider::gitlab::GitLabProvider;

pub trait OwnerRepoSource {
    fn owner_str(&self) -> &str;
    fn repo_str(&self) -> &str;
}

impl OwnerRepoSource for GenericProvider {
    fn owner_str(&self) -> &str {
        self.owner().as_str()
    }
    fn repo_str(&self) -> &str {
        self.repo().as_str()
    }
}

impl OwnerRepoSource for GitLabProvider {
    fn owner_str(&self) -> &str {
        self.owner().as_str()
    }
    fn repo_str(&self) -> &str {
        self.repo().as_str()
    }
}

#[derive(serde::Serialize)]
pub struct OwnerRepo<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

#[derive(serde::Serialize)]
pub struct GitURL<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<&'a str>,
    pub path: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_repo: Option<OwnerRepo<'a>>,
}

pub fn gu2json2wtr_opt<W>(
    wtr: &mut W,
    gu: &GitUrl,
    orepo: Option<OwnerRepo>,
) -> Result<(), io::Error>
where
    W: Write,
{
    let port: Option<u16> = gu.port();
    let scheme: Option<&str> = gu.scheme();
    let user: Option<&str> = gu.user();
    let host: Option<&str> = gu.host();
    let path: &str = gu.path();
    let g = GitURL {
        port,
        scheme,
        user,
        host,
        path,
        owner_repo: orepo,
    };
    serde_json::to_writer(wtr.by_ref(), &g)?;
    writeln!(wtr)
}

pub fn gu2json2wtr_o<W, O>(wtr: &mut W, gu: &GitUrl, owner_repo: O) -> Result<(), io::Error>
where
    W: Write,
    O: OwnerRepoSource,
{
    let owner: &str = owner_repo.owner_str();
    let repo: &str = owner_repo.repo_str();
    let or = OwnerRepo { owner, repo };
    gu2json2wtr_opt(wtr, gu, Some(or))
}

pub fn gu2json2wtr<W>(wtr: &mut W, gu: &GitUrl) -> Result<(), io::Error>
where
    W: Write,
{
    let ogh: Option<GenericProvider> = gu.provider_info().ok();
    let ogl: Option<GitLabProvider> = gu.provider_info().ok();
    match (ogh, ogl) {
        (None, None) => gu2json2wtr_opt(wtr, gu, None),
        (None, Some(gl)) => gu2json2wtr_o(wtr, gu, gl),
        (Some(gh), None) => gu2json2wtr_o(wtr, gu, gh),
        (Some(gh), Some(_)) => gu2json2wtr_o(wtr, gu, gh),
    }
}

pub fn string2url(s: String) -> Result<GitUrl, io::Error> {
    GitUrl::parse(&s).map_err(io::Error::other)
}

pub fn giturlsink4json<W>(mut wtr: W) -> impl FnMut(GitUrl) -> Result<(), io::Error>
where
    W: Write,
{
    move |u: GitUrl| gu2json2wtr(&mut wtr, &u)
}

pub fn strings2parsed2jsons2wtr<I, W>(strings: I, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
    W: Write,
{
    let mut jwtr = giturlsink4json(&mut wtr);
    for rline in strings {
        let line: String = rline?;
        let gu: GitUrl = string2url(line)?;
        jwtr(gu)?;
    }
    drop(jwtr);

    wtr.flush()
}

pub fn stdin2strings2jsons2stdout() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    strings2parsed2jsons2wtr(io::stdin().lock().lines(), BufWriter::new(&mut ol))?;
    ol.flush()
}
