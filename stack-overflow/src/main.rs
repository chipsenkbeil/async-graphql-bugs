use std::convert::Infallible;
use warp::{reply::Reply, Filter};

#[derive(Default)]
pub struct Page {
    contents: Vec<usize>,
}

#[async_graphql::Object]
impl Page {
    async fn contents_ids(&self) -> &[usize] {
        &self.contents
    }

    async fn contents(&self) -> Vec<BlockElement> {
        Vec::new()
    }
}

#[derive(async_graphql::Union)]
pub enum Element {
    #[graphql(flatten)]
    Block(BlockElement),
}

#[derive(async_graphql::Union)]
pub enum BlockElement {
    Blockquote(Blockquote),
}

#[derive(Default)]
pub struct Blockquote {
    lines: Vec<String>,
    page: usize,
    parent: Option<usize>,
}

#[async_graphql::Object]
impl Blockquote {
    async fn lines(&self) -> &[String] {
        &self.lines
    }

    async fn page_id(&self) -> usize {
        self.page
    }

    async fn page(&self) -> Page {
        Page {
            contents: Vec::new(),
        }
    }

    async fn parent_id(&self) -> Option<usize> {
        self.parent.as_ref().copied()
    }

    async fn parent(&self) -> Option<Element> {
        None
    }
}

#[derive(Default)]
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn page(&self) -> Page {
        Page::default()
    }

    async fn element(&self) -> Element {
        Element::Block(BlockElement::Blockquote(Blockquote::default()))
    }

    async fn block_element(&self) -> BlockElement {
        BlockElement::Blockquote(Blockquote::default())
    }

    async fn blockquote(&self) -> Blockquote {
        Blockquote::default()
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
