use surrealdb::{Surreal};
use surreal_query_builder::sql;
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Serialize, Deserialize)]
struct UserInfo {
    name: String,
    age: u8,
    coordinates: u8,
}


#[derive(Serialize, Deserialize)]
#[table = "user"]
struct User {
    id: u64,
    // #[serde(flatten)]
    // user_info: UserInfo,
    //

    // Fields in the database
    name: String,
    age: u8,
    coordinates: u8,
}

#[derive(Serialize, Deserialize)]
struct NewUser( USer);

#[derive(Serialize, Deserialize)]
struct NewUser{ 
    #[serde(flatten)]
    user_info: USer, 
    something_else: String 
};
type NewUser = USer;


struct NewUser {
    name: String,
    age: u8,
    coordinates: u8,
}

pick!(User, NewUser; [name, age, coordinates]))
omit!(User, NewUser; [name, age, coordinates]))

object! {}
object_partial! {}
k


// Query
// "SELECT *, name, age, address.coordinates AS coordinates FROM person
// WHERE ->knows->person->(knows WHERE influencer = $influencer) TIMEOUT 5s
// "
//
// PROS
// - We can validate/check the return type field names against what was selected in the query using
// field name or field alias (when field name has complex characters, uses functions or accesses
// foreign tables).
// - We can default all field types to serde_json::Value and allow the user to override the type
//
// - defaulting every field type to serde_json::Value might provide quick escape for adhoc queries.
// Can be especially useful if you dont have to share the return type struct to different service
// or client, so you're mainly just concerned about the serialized data.
// Rebut: We can also achieve this just just returning a top-level serde_json::Value from the select method.
// 


// CONS
// - Ergonomics: defaulting every field type to serde_json::Value might provide quick escape for adhoc queries,
// but it does not provide much utility/ good developer experience to know what type to expect
// especially if sharing same struct across multiple queries/services/clients.
//
// - Additional mental overhead: One more thing to learn for the developer. But this can be eased up a little by providing a macro to
// generate the return type struct.

select_infer!(User, "SELECT *, name, age, address.coordinates AS coordinates FROM person
        WHERE ->knows->person->(knows WHERE influencer = $influencer) TIMEOUT 5s", {
    person: Person, // Require this from the dev
    name: serde_json::Value,
    age: serde_json::Value,
    coordinates: Coordinates,
})

// Return Type
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(flatten)]
    person: Person, // Require this from the dev
    name: serde_json::Value,
    age: serde_json::Value,
    coordinates: Coordinates,
}

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

struct SomeSelection {
    name: String,
    age: u8,
    coordinates: u8,
}

impl User {
    pub fn select_variables<'a>(db: Surreal<>, variables: Variables<'a>) -> !{
        let q = query(
            "SELECT *, name, age, count(), array::sum(address.coordinates) AS coordinates FROM person
            WHERE ->knows->person->(knows WHERE influencer = $influencer) TIMEOUT 5s",
            variables
        ).bind("influencer", variables.influencer)


        ;

    }
}

fn test_it(){
    let variables = Variables {
        influencer: &true,
    };
    User::select(variables);
}
