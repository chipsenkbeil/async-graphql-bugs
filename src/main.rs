use entity::*;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{reply::Reply, Filter};

#[simple_ent]
#[derive(AsyncGraphqlEnt, AsyncGraphqlEntFilter)]
pub struct Page {
    #[ent(edge(policy = "deep", wrap), ext(async_graphql(filter_untyped)))]
    contents: Vec<BlockElement>,
}

#[simple_ent]
#[derive(async_graphql::Union)]
pub enum Element {
    #[ent(wrap)]
    #[graphql(flatten)]
    Block(BlockElement),
}

#[simple_ent]
#[derive(async_graphql::Union, Debug)]
pub enum BlockElement {
    Blockquote(Blockquote),
}

#[simple_ent]
#[derive(AsyncGraphqlEnt, AsyncGraphqlEntFilter)]
pub struct Blockquote {
    #[ent(field, ext(async_graphql(filter_untyped)))]
    region: Region,
    lines: Vec<String>,

    /// Page containing the blockquote
    #[ent(edge)]
    page: Page,

    /// Parent element to this blockquote
    #[ent(edge(policy = "shallow", wrap), ext(async_graphql(filter_untyped)))]
    parent: Option<Element>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    async_graphql::SimpleObject,
    Serialize,
    Deserialize,
    ValueLike,
)]
pub struct Region {
    offset: usize,
    len: usize,
    position: Option<Position>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    async_graphql::SimpleObject,
    Serialize,
    Deserialize,
    ValueLike,
)]
pub struct Position {
    start: LineColumn,
    end: LineColumn,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    async_graphql::SimpleObject,
    Serialize,
    Deserialize,
    ValueLike,
)]
pub struct LineColumn {
    line: usize,
    column: usize,
}

#[derive(Default)]
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn page(&self) -> Page {
        Page::build().contents(Vec::new()).finish().unwrap()
    }

    async fn element(&self) -> Element {
        let blockquote = Blockquote::build()
            .region(Region::default())
            .lines(Vec::new())
            .page(0)
            .parent(None)
            .finish()
            .unwrap();
        Element::Block(BlockElement::Blockquote(blockquote))
    }

    async fn block_element(&self) -> BlockElement {
        let blockquote = Blockquote::build()
            .region(Region::default())
            .lines(Vec::new())
            .page(0)
            .parent(None)
            .finish()
            .unwrap();
        BlockElement::Blockquote(blockquote)
    }

    async fn blockquote(&self) -> Blockquote {
        Blockquote::build()
            .region(Region::default())
            .lines(Vec::new())
            .page(0)
            .parent(None)
            .finish()
            .unwrap()
    }
}

pub type Schema =
    async_graphql::Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

#[tokio::main]
async fn main() {
    let schema = Schema::build(
        Query::default(),
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .finish();

    let routes = warp::any().and(warp::path("/graphql").and(
        async_graphql_warp::graphql(schema).and_then(
            |(schema, request): (Schema, async_graphql::Request)| async move {
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(warp::reply::json(&resp).into_response())
            },
        ),
    ));

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
