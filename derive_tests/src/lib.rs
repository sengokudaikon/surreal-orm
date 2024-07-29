use serde::{Serialize, Deserialize};
use surrealdb::sql;
// struct User;
struct SelectionMeta {
    query_string: String,
    selected_field_type_metadata: String, // Tokenstream
    variables_interpolated: String, // Tokenstream
}

// struct Coordinates;
// // SELECT * FROM person WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
// select_infer!(PersonSelect, "SELECT * person", {
//     person: Person
// });
//
// #[derive(Serialize, Deserialize)]
// struct PersonSelect {
//     #[serde(flatten)]
//     person: Person,
// }
//
//
// select_infer!(User, "SELECT *, name, age, address.coordinates AS coordinates FROM person
// WHERE ->knows->person->(knows WHERE influencer = $influencer) TIMEOUT 5s
// ", {
//     coorinates: Coordinates
// })
//
// #[derive(Serialize, Deserialize)]
// struct User {
//     #[serde(flatten)]
//     person: Person,
//     name: serde_json::Value,
//     age: serde_json::Value,
//     coordinates: Coordinates,
// }

trait ToValue {
    fn to_value(self) -> sql::Value;
}

impl<T: Into<sql::Value>> ToValue for T {
    fn to_value(self) -> sql::Value {
        let value: sql::Value = self.into();
        value
        // serde_json::Value::String(self.into().to_string())
    }
}

struct Variables<'a> {
    influencer: &'a dyn ToValue,
}

// impl User {
//     pub fn select(variables: String)
// }

// SELECT name FROM person WITH NOINDEX WHERE job = 'engineer' AND gender = 'm';
// SELECT * FROM person WITH INDEX ft_email WHERE email = 'tobie@surrealdb.com' AND company = 'SurrealDB';
// SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL;
// SELECT * FROM person WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
// SELECT * FROM article WHERE author.age < 30 FETCH author;
// SELECT *, artist.email FROM review FETCH artist;
// SELECT * FROM user LIMIT 50 START 50;
// SELECT array::group(tags) AS tags FROM article GROUP ALL;
// SELECT count() AS number_of_records FROM person GROUP ALL;
// SELECT * FROM song ORDER BY artist ASC, rating DESC;
// SELECT * FROM profile WHERE count(->experience->organisation) > 3;
// -- Conditional filtering based on graph edge properties
// SELECT * FROM person WHERE ->(reaction WHERE type='celebrate')->post;
//
// -- Conditional filtering with boolean logic
// SELECT * FROM user WHERE (admin AND active) OR owner = true;
// SELECT * FROM (SELECT age >= 18 AS adult FROM user) WHERE adult = true;
// SELECT * FROM [3648937, "test", person:lrym5gur8hzws72ux5fa, person:4luro9170uwcv1xrfvby];
// SELECT * FROM user:tobie, user:jaime, company:surrealdb;
// SELECT * FROM temperature:['London', '2022-08-29T08:03:39']..['London', '2022-08-29T08:09:31'];
// SELECT *, (SELECT * FROM events WHERE type = 'activity' LIMIT 5) AS history FROM user;
// LET $history = SELECT * FROM events WHERE type = 'activity' LIMIT 5;
// SELECT * FROM person WHERE ->(reacted_to WHERE type='celebrate')->post;
// SELECT address[WHERE active = true] FROM person;
// SELECT { weekly: false, monthly: true } AS `marketing settings` FROM user;
// SELECT (( celsius * 1.8 ) + 32) AS fahrenheit FROM temperature;
// SELECT array::group(tags) AS tags FROM article GROUP ALL;
// SELECT array::distinct(tags) FROM article;
// SELECT address.coordinates[0] AS latitude FROM person;
// SELECT address.coordinates AS coordinates FROM person;
// SELECT address.*.coordinates AS coordinates FROM person;
// SELECT address.city FROM person;
// SELECT name, address, email FROM person:tobie;

fn select_user() {
    todo!();
}
