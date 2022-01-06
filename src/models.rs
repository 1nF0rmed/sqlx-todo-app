use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
  pub id: String,
  pub title: String,
  pub descp: String,
  pub completed: bool,
}
