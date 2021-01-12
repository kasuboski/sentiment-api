#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rust_bert::pipelines::sentiment::{SentimentModel, SentimentPolarity};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct SentimentResult {
    polarity: String,
}

#[derive(Serialize, Deserialize)]
struct SentimentRequest {
    data: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ResultsResponse {
  data: Vec<SentimentResult>,
}

#[post("/", format = "json", data = "<request>")]
fn index(
  request: Json<SentimentRequest>,
  shared_sentiment: State<Mutex<SentimentModel>>,
) -> Option<Json<ResultsResponse>> {
  let sentiment_classifier = shared_sentiment.lock().ok()?;
  //    Define input
  let input: Vec<&str> = request.data.iter().map(|i| i.as_str()).collect();

  //    Run model
  let output = sentiment_classifier.predict(&input);

  let results = output
    .iter()
    .map(|sentiment| {
        let polarity = match sentiment.polarity {
            SentimentPolarity::Positive => "positive".to_string(),
            SentimentPolarity::Negative => "negative".to_string(),
        };

        SentimentResult { polarity: polarity }
    })
    .collect();

  Some(Json(ResultsResponse { data: results }))
}

#[catch(404)]
fn not_found() -> JsonValue {
  json!({
    "status": "error",
    "reason": "Resource was not found."
  })
}

fn main() {
  let sentiment_classifier =
    SentimentModel::new(Default::default()).expect("Couldn't start model");
  // warm up the classifier 😉
  sentiment_classifier.predict(&["positive"]);

  rocket::ignite()
    .manage(Mutex::new(sentiment_classifier))
    .mount("/", routes![index])
    .register(catchers![not_found])
    .launch();
}
