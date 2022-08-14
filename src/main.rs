use scraper::{Html, Selector};
use ureq::Agent;

fn main() {
    let client = Client::new(UReqClient::new());
    let Response { status: _, body } = client
        .get("https://example.com/")
        .unwrap();

    let _verification = select_on_document(&body, "form>input[name~=__RequestVerificationToken]", Attr::Value);
}

pub enum Attr<'a> {
    Value,
    Content,
    Custom(&'a str)
}
impl Attr<'_> {
    fn as_str(&self) -> &str {
        match self {
            Attr::Value => "value",
            Attr::Content => "content",
            Attr::Custom(str) => str,
        }
    }
}

pub fn select_on_document<'a>(body: &str, selector: &str, attr: Attr) -> Option<String> {
    let html = Html::parse_document(body);
    println!("got html");
    let selector = Selector::parse(selector).ok()?;
    println!("got selector: {selector:?}");
    let element = html.select(&selector).next()?.value();
    println!("got element {element:?}");

    element.attr(attr.as_str()).map(|v| v.to_string())
}

pub trait IClient {
    type Error;
    fn get(&self, path: &str) -> Result<Response, Self::Error>;
    fn add_cookie(&mut self, cookie: &str, url: &str);
}
pub trait ResponseErr: std::fmt::Debug {}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    status: u16,
    body: String,
}

pub struct Client<C: IClient> {
    client: C,
}

impl<C: IClient> Client<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn get(&self, path: &str) -> Result<Response, impl ResponseErr>
    where
        <C as IClient>::Error: ResponseErr,
    {
        self.client.get(path)
    }

    pub fn add_coockie(&mut self, coockie: &str) -> &mut Self {

        self
    }
}

#[derive(Debug)]
pub enum UReqResponseErr {
    Client(ureq::Error),
    IO(std::io::Error),
}
impl From<ureq::Error> for UReqResponseErr {
    fn from(e: ureq::Error) -> Self {
        Self::Client(e)
    }
}
impl From<std::io::Error> for UReqResponseErr {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
impl ResponseErr for UReqResponseErr {}

pub struct UReqClient {
    pub agent: Agent,
}

impl IClient for UReqClient {
    type Error = UReqResponseErr;
    fn get(&self, url: &str) -> Result<Response, Self::Error> {
        let resp = self.agent.get(url).call()?;

        Ok(Response {
            status: resp.status(),
            body: resp.into_string()?,
        })
    }

    fn add_cookie(&mut self, cookie: &str, url: &str) {
        let state = self.agent.state;

            .cookie_tin();

            // cookie_store();
        // store.parse(cookie, url::Url::parse(url).as_ref().unwrap());

    }
}

impl UReqClient {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    // #colCenter > form:nth-child(1) > input:nth-child(2)
    // html body div.colmask.threecol div.colmid div.colleft div#colCenter.col1 form input
    // /html/body/div[2]/div/div/div[1]/form/input
    // __RequestVerificationToken

    #[test]
    fn select_request_verification() {
        assert_eq!(
            select_on_document(
                r#"
                <body>
                <form>
                <input name="please_not_this", value="doNotdoIt">
                <input name="__RequestVerificationToken" type="hidden"
                value="CfDJ8HoKEkgJt5">
                </form>
                </body>
                "#,
                "form>input[name~=__RequestVerificationToken]",
                Attr::Value
            ),
            Some("CfDJ8HoKEkgJt5".to_string())
        );
    }

    #[test]
    fn get_website() {
        let resp = ureq::get("https://example.com/").call().unwrap();
        let status = resp.status();
        let body = resp.into_string().unwrap();

        assert_eq!(200, status);
        assert_eq!(
            select_on_document(&body, "meta[name~=viewport]", Attr::Content),
            Some("width=device-width, initial-scale=1".to_owned())
        );
    }

    #[test]
    fn get_website_custom() {
        let client = UReqClient::new();
        let client = Client::new(client);
        let Response { status, body } = client.get("https://example.com/").unwrap();

        assert_eq!(200, status);
        assert_eq!(
            select_on_document(&body, "meta[name~=viewport]", Attr::Content),
            Some("width=device-width, initial-scale=1".to_owned())
        );
    }

    #[test]
    fn attr() {
        assert_eq!(Attr::Custom("test").as_str(), "test");
    }
    //
    // TODO:
    // 1 -- get from website and parse the html
    // 2 -- parse the document
    // 3 -- post login request
    // 4 -- get / post the other request wanted
    // 5 -- see if I can change it to be more abstract behind a trait?

    // TODO: Make a format that can change these request together and select
    // data with css
}
