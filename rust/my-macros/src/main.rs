// For testing macros made here

use my_macros::{HelloMacro, KeyNamesGetter, MyTrait};
use serde::{Deserialize, Serialize};
// use serde::{Serialize, Deserialize};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(MyTrait)]
#[my_trait(answer = 50, level = "high")]
struct Foo {
    group: String,
}

// #[derive(MyTrait)]
// #[my_trait(answer = 0, name = "lowo")]
// struct Bar;

#[derive(MyTrait)]
#[my_trait(answer = 20, level = "low")]
struct Bar {
    name: String,
    age: u8,
}

// #[my_crate(lorem(dolor = "Hello", sit))]
#[derive(KeyNamesGetter, Serialize, Deserialize)]
// #[mongoye(typee = "Hello")]
// #[mongoye(typee = "Hello", case = "snake")]
#[serde(rename_all = "camelCase")]
pub struct ConsumingType {
    #[serde(rename = "lowo_cool")]
    pub name_of_me: String,
    #[serde(rename = "lmsar")]
    // #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
    pub age: u8,
}

pub mod pp {
    pub const gg: &str = "34";
}

struct Make {
    name: &'static str,
}

fn main() {
    let ConsumingTypeKeyNames {
        lowo_cool, lmsar, ..
    } = ConsumingType::get_field_names();

    // println!("rerezzzzzzz{name_of_me}, {lmsar}")
    println!("rere{lowo_cool}, {lmsar}")
    // ConsumingType::get_field_names();
    // Pancakes::hello_macro();
    // println!("Hello, world!");
    // println!("Foo::answer() = {}", Foo::answer());
    // println!("Bar::answer() = {}", Bar::answer());

    // println!("Foo::level() = {}", Foo::level());
    // println!("Bar::level() = {}", Bar::level());
}

#[test]
fn default() {
    assert_eq!(Foo::answer(), 50);
    assert!(Foo::level().contains("High"));
    assert!(!Foo::level().contains("Low"));
}

#[test]
fn getter() {
    assert_eq!(Bar::answer(), 20);
    assert!(Bar::level().contains("Low"));
}

#[test]
fn keys_getter_1() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consumer {
        #[serde(rename = "lowo_cool")]
        pub name_of_me: String,

        #[serde(rename = "age_count")]
        pub age: u8,
    }

    let ConsumerKeyNames {
        lowo_cool,
        age_count,
    } = Consumer::get_field_names();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age_count, "age_count");
}

#[test]
fn keys_getter_4() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "camelCase")]
    pub struct Consumer {
        pub name_of_me: String,

        pub age: u8,
    }

    let ConsumerKeyNames { nameOfMe, age } = Consumer::get_field_names();

    assert_eq!(nameOfMe, "nameOfMe");
    assert_eq!(age, "age");
}

#[test]
fn keys_getter_5() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub nameOfMe: String,

        pub age: u8,
    }

    let ConsumerKeyNames { name_of_me, age } = Consumer::get_field_names();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age, "age");
}

#[test]
fn keys_getter_6() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        pub ageCount: u8,
    }

    let ConsumerKeyNames {
        name_of_me,
        age_count,
    } = Consumer::get_field_names();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age_count, "age_count");
}
