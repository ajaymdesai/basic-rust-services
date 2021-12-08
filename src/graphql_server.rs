// use std::{convert::Infallible, sync::Arc};
// extern crate pretty_env_logger;
//
// use hyper::{
//   server::Server,
//   service::{make_service_fn, service_fn},
//   Body, Method, Response, StatusCode,
// };
//
// use juniper::{EmptyMutation, EmptySubscription, RootNode, GraphQLObject, GraphQLType, Registry, DefaultScalarValue, GraphQLValue};
// use juniper::meta::MetaType;
//
// #[derive(GraphQLObject)]
// pub struct HelloWorld {
//   greeting: String,
// }
//
// impl HelloWorld {
//   fn new(name: String) -> HelloWorld {
//     return HelloWorld{ greeting: format!("Hello, {}!", name) };
//   }
// }
//
// struct Context {
//   default_greeting: String,
// }
//
// // To make our context usable by Juniper, we have to implement a marker trait.
// impl juniper::Context for Context {}
//
// struct Query;
//
// #[graphql_object(
//     // Here we specify the context type for the object.
//     // We need to do this in every type that
//     // needs access to the context.
//     context = Context,
// )]
// impl Query {
//     fn apiVersion() -> &'static str {
//         "1.0"
//     }
//
//     // Arguments to resolvers can either be simple types or input objects.
//     // To gain access to the context, we specify a argument
//     // that is a reference to the Context type.
//     // Juniper automatically injects the correct context here.
//     fn hello(context: &Context, id: String) -> FieldResult<HelloWorld> {
//         let msg = HelloWorld::new(id);
//         // Return the result.
//         Ok(msg)
//     }
// }
//
// struct Mutation;
//
// #[graphql_object(
//     context = Context,
// )]
// impl Mutation {
// }
// // A root schema consists of a query, a mutation, and a subscription.
// // Request queries can be executed against a RootNode.
// type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
//
// #[tokio::main]
// async fn main() {
//   pretty_env_logger::init();
//
//   let addr = ([127, 0, 0, 1], 3000).into();
//
//   let root_node = Arc::new(RootNode::new(
//     Query,
//     EmptyMutation::<String>::new(),
//     EmptySubscription::<String>::new(),
//   ));
//
//   let ctx= Context {default_greeting: String::from("Hello")};
//   let new_service = make_service_fn(move |_| {
//     let root_node = root_node.clone();
//
//     async {
//       Ok::<_, hyper::Error>(service_fn(move |req| {
//         let root_node = root_node.clone();
//         let ctx = ctx.clone();
//         async {
//           Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
//             (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
//             (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
//               juniper_hyper::graphql(root_node, ctx, req).await
//             }
//             _ => {
//               let mut response = Response::new(Body::empty());
//               *response.status_mut() = StatusCode::NOT_FOUND;
//               response
//             }
//           })
//         }
//       }))
//     }
//   });
//
//   let server = Server::bind(&addr).serve(new_service);
//   println!("Listening on http://{}", addr);
//
//   if let Err(e) = server.await {
//     eprintln!("server error: {}", e)
//   }
// }
// Only needed due to 2018 edition because the macro is not accessible.
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, Variables, graphql_value,
};

// Arbitrary context data.
struct Ctx();

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    fn hello(_context: &Ctx) -> FieldResult<String> {

        Ok(format!("Hello World!"))
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn main() {
    // Create a context object.
    let ctx = Ctx();

    // Run the executor.
    let (res, _errors) = juniper::execute_sync(
        "query { hello }",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();

    // Ensure the value matches.
    assert_eq!(
        res,
        graphql_value!({
            "hello": "Hello World!",
        })
    );
}
