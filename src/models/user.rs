pub struct User {
  pub id: i32,

  pub email: String,

  pub name: String,
}
#[juniper::object(description = "An user")]
impl User {
  pub fn id(&self) -> i32 {
    self.id
  }
  pub fn email(&self) -> &str {
    self.email.as_str()
  }
  pub fn name(&self) -> &str {
    self.name.as_str()
  }
}