use base_actor::BaseActor;
use file::image::Image;
use schema::personas;
use sql_types::PostVisibility;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "personas"]
pub struct Persona {
    id: i32,
    default_visibility: PostVisibility,
    is_searchable: bool,
    avatar: Option<i32>, // foreign key to Image
    shortname: String,   // wtf is a SlugField
    base_actor: i32,     // foreign key to BaseActor
}

impl Persona {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn default_visibility(&self) -> PostVisibility {
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
    default_visibility: PostVisibility,
    is_searchable: bool,
    avatar: Option<i32>,
    shortname: String,
    base_actor: i32,
}

impl NewPersona {
    pub fn new(
        default_visibility: PostVisibility,
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
