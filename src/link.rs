use std::fmt;
use std::io::Write;
use std::str::FromStr;
use std::error::Error as StdError;

use diesel::backend::Backend;
use diesel::serialize;
use diesel::deserialize;
use diesel::sql_types::Text;
use url::Url;

use base_post::BasePost;
use schema::links;

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum Lang {
    EnUs,
    EnUk,
    EnAu,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Lang::EnUs => write!(f, "EnUs"),
            Lang::EnUk => write!(f, "EnUk"),
            Lang::EnAu => write!(f, "EnAu"),
        }
    }
}

impl FromStr for Lang {
    type Err = LangParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EnUs" => Ok(Lang::EnUs),
            "EnUk" => Ok(Lang::EnUk),
            "EnAu" => Ok(Lang::EnAu),
            _ => Err(LangParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for Lang
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Lang
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Lang>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LangParseError;

impl fmt::Display for LangParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse Lang")
    }
}

impl StdError for LangParseError {
    fn description(&self) -> &str {
        "Failed to parse Lang"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "links"]
pub struct Link {
    id: i32,
    href: Url, // max_length: 2048
    href_lang: Lang,
    height: u32,
    width: u32,
    preview: String,
    base_post: i32, // foreign key to BasePost
}

impl Link {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn href(&self) -> &Url {
        &self.href
    }

    pub fn href_lang(&self) -> Lang {
        self.href_lang
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn preview(&self) -> &str {
        &self.preview
    }

    pub fn base_post(&self) -> i32 {
        self.base_post
    }
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    href: String,
    href_lang: Lang,
    height: i32,
    width: i32,
    preview: String,
    base_post: i32,
}

impl NewLink {
    pub fn new(
        href: Url,
        href_lang: Lang,
        height: u32,
        width: u32,
        preview: String,
        base_post: &BasePost,
    ) -> Self {
        NewLink {
            href: href.to_string(),
            href_lang,
            height: height as i32,
            width: width as i32,
            preview,
            base_post: base_post.id(),
        }
    }
}
