use juniper::{EmptyMutation, RootNode};

mod user;
use crate::models::user::{User};

pub struct QueryRoot;
#[juniper::object]
impl QueryRoot {
  fn users() -> Vec<User> {
    vec![
      User {
        id: 1,
        email: "beatyshot@gmail.com".to_owned(),
        name: "Yuki Yami".to_owned(),
      },
      User {
        id: 2,
        email: "beatyneko@gmail.com".to_owned(),
        name: "Eric".to_owned(),
      }
    ]
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}