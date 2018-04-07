use std::error::Error as StdError;
use std::fmt;
use std::io::Write;
use std::str::FromStr;

use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::Text;

use base_actor::BaseActor;
use file::image::Image;
use schema::personas;

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum Visibility {
    Public,
    FollowersOnly,
    FriendsOnly,
    ListedPeopleOnly,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Visibility::Public => write!(f, "PUB"),
            Visibility::FollowersOnly => write!(f, "FL"),
            Visibility::FriendsOnly => write!(f, "MUT"),
            Visibility::ListedPeopleOnly => write!(f, "LIST"),
        }
    }
}

impl FromStr for Visibility {
    type Err = VisibilityParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PUB" => Ok(Visibility::Public),
            "FL" => Ok(Visibility::FollowersOnly),
            "MUT" => Ok(Visibility::FriendsOnly),
            "LIST" => Ok(Visibility::ListedPeopleOnly),
            _ => Err(VisibilityParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for Visibility
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Visibility
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Visibility>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibilityParseError;

impl fmt::Display for VisibilityParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse Visibility")
    }
}

impl StdError for VisibilityParseError {
    fn description(&self) -> &str {
        "Failed to parse Visibility"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "personas"]
pub struct Persona {
    id: i32,
    default_visibility: Visibility,
    is_searchable: bool,
    avatar: Option<i32>, // foreign key to Image
    shortname: String,   // wtf is a SlugField
    base_actor: i32,     // foreign key to BaseActor
}

impl Persona {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn default_visibility(&self) -> Visibility {
        self.default_visibility
    }

    pub fn is_searchable(&self) -> bool {
        self.is_searchable
    }

    pub fn avatar(&self) -> Option<i32> {
        self.avatar
    }

    pub fn shortname(&self) -> &str {
        &self.shortname
    }

    pub fn base_actor(&self) -> i32 {
        self.base_actor
    }
}

#[derive(Insertable)]
#[table_name = "personas"]
pub struct NewPersona {
    default_visibility: Visibility,
    is_searchable: bool,
    avatar: Option<i32>,
    shortname: String,
    base_actor: i32,
}

impl NewPersona {
    pub fn new(
        default_visibility: Visibility,
        is_searchable: bool,
        avatar: Option<&Image>,
        shortname: String,
        base_actor: &BaseActor,
    ) -> Self {
        NewPersona {
            default_visibility,
            is_searchable,
            avatar: avatar.map(|a| a.id()),
            shortname,
            base_actor: base_actor.id(),
        }
    }
}
