#[path = "./graphql_resolver.rs"]
mod graphql_resolver;

use juniper::{FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject};
use mongodb::Client;

trait Node {
    fn id(&self) -> &str;
}

pub fn get_streamer(id: &str) -> Option<Streamer> {
    match id {
        _ if id == "1" => Some(Streamer {
            id: String::from("ozer"),
            firstname: String::from("cevikaslan"),
        }),
        _ => None,
    }
}

pub fn get_session(id: &str) -> Option<Session> {
    match id {
        _ if id == "2" => Some(Session {
            id: String::from("ozer"),
            streamer_id: String::from("cevikaslan"),
            description: String::from("cevikaslan"),
            title: String::from("cevikaslan"),
            categories: vec![],
        }),
        _ => None,
    }
}

juniper::graphql_interface!(<'a> &'a dyn Node: Context as "Node" |&self| {
    instance_resolvers: |_| {
        Streamer => get_streamer(self.id()),
        Session => get_session(self.id()),
    }

    field id() -> &str { self.id() }
});

#[derive(GraphQLEnum, Debug)]
pub enum Category {
    Programming,
    VideoGaming,
    Basketball,
    Soccer,
    Music,
}

pub struct Session {
    pub id: String,
    pub streamer_id: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<Category>,
}

#[juniper::object(name = "Session", interfaces = [&dyn Node], Context = Context, description = "Session")]
impl Session {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    #[graphql(name = "streamerId")]
    fn streamer_id(&self) -> &str {
        self.streamer_id.as_str()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn categories(&self) -> Vec<Category> {
        vec![Category::Programming]
    }
}

pub struct Streamer {
    pub id: String,
    pub firstname: String,
}

#[juniper::object(name = "Streamer",
    Context = Context,
    interfaces = [&dyn Node],
    description = "Streamer")]
impl Streamer {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn firstname(&self) -> &str {
        self.firstname.as_str()
    }
}

impl Node for Streamer {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl Node for Session {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Input for creating new streamer...")]
struct NewStreamer {
    id: String,
    firstname: String,
    lastname: String,
}

#[derive(Clone)]
pub struct Context {
    pub db: Client,
}

impl juniper::Context for Context {}

pub struct MutationRoot;

#[juniper::object(Context = Context,
description = "Mutation Root",)]
impl MutationRoot {
    #[graphql(name = "createStreamer")]
    fn create_streamer(context: &Context, new_streamer: NewStreamer) -> FieldResult<Streamer> {
        Ok(graphql_resolver::create_streamer(&context.db.clone()))
    }
}

pub struct QueryRoot {}

#[juniper::object(Context = Context,
description = "Query Root")]
impl QueryRoot {
    #[graphql(description = "get a streamer")]
    fn streamer(context: &Context, id: String) -> FieldResult<Streamer> {
        Ok(Streamer {
            id: String::from("id"),
            firstname: String::from("ozer"),
        })
    }

    #[graphql(description = "Node")]
    fn node(context: &Context, id: String) -> FieldResult<Option<&dyn Node>> {
        let streamer = Streamer {
            id: String::from("id"),
            firstname: String::from("ozer"),
        };

        Ok(None)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
