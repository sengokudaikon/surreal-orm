use chrono::{DateTime, Utc};
use geo::Point;
use geo::Polygon;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surrealdb::engine::local::Mem;
use surrealdb::sql;
use surrealdb::Surreal;
use surrealdb_models::{alien_schema, spaceship_schema, Alien, SpaceShip, Weapon};
use surrealdb_orm::statements::order;
use surrealdb_orm::*;

// #[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[surrealdb(table_name = "alien")]
// pub struct Alien {
//     id: sql::Thing,
//     name: String,
//     age: u8,
//     created: DateTime<Utc>,
//     life_expectancy: Duration,
//     territory_area: Polygon,
//     home: Point,
//     tags: Vec<String>,
//     // database type attribute is autogenerated for all links of the struct. But you can also provide it
//     #[surrealdb(link_self = "Alien")]
//     ally: LinkSelf<Alien>,
//
//     #[surrealdb(link_one = "Weapon")]
//     weapon: LinkOne<Weapon>,
//
//     // Again, we dont have to provide the type attribute, it can auto detect
//     #[surrealdb(link_many = "SpaceShip")]
//     space_ships: LinkMany<SpaceShip>,
// }

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien_with_explicit_attributes")]
pub struct AlienWithExplicitAttributes {
    id: sql::Thing,
    name: String,
    age: u8,
    created: DateTime<Utc>,
    life_expectancy: Duration,
    territory_area: Polygon,
    home: Point,
    tags: Vec<String>,
    // database type attribute is autogenerated for all links of the struct. But you can also provide it
    #[surrealdb(
        link_self = "AlienWithExplicitAttributes",
        type = "record(alien_with_explicit_attributes)"
    )]
    ally: LinkSelf<AlienWithExplicitAttributes>,

    #[surrealdb(link_one = "Weapon", type = "record(weapon)")]
    weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    #[surrealdb(
        link_many = "SpaceShip",
        type = "array",
        content_type = "record(space_ship)"
    )]
    space_ships: LinkMany<SpaceShip>,
}
#[derive(SurrealdbEdge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "visits")]
pub struct Visits<In: SurrealdbNode, Out: SurrealdbNode> {
    id: sql::Thing,
    #[serde(rename = "in")]
    in_: LinkOne<In>,
    out: LinkOne<Out>,
    time_visited: Duration,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "planet")]
pub struct Planet {
    id: sql::Thing,
    name: String,
    area: Polygon,
    population: u64,
    created: DateTime<Utc>,
    tags: Vec<String>,
}

#[tokio::test]
async fn test_node_atttributes_auto_inferred() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    Alien::define_fields();
    assert_eq!(
        Alien::define_table().to_raw().build(),
        "DEFINE TABLE alien;"
    );

    assert_eq!(
        Alien::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE alien;
DEFINE FIELD name ON TABLE alien;
DEFINE FIELD age ON TABLE alien;
DEFINE FIELD created ON TABLE alien;
DEFINE FIELD lifeExpectancy ON TABLE alien;
DEFINE FIELD linePolygon ON TABLE alien;
DEFINE FIELD territoryArea ON TABLE alien;
DEFINE FIELD home ON TABLE alien;
DEFINE FIELD tags ON TABLE alien;
DEFINE FIELD ally ON TABLE alien;
DEFINE FIELD weapon ON TABLE alien;
DEFINE FIELD spaceShips ON TABLE alien;
DEFINE FIELD spaceShips.* ON TABLE alien TYPE record (space_ship);"
    );

    Ok(())
}

#[tokio::test]
async fn test_node_atttributes_explicit() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    Alien::define_fields();
    assert_eq!(
        AlienWithExplicitAttributes::define_table().to_raw().build(),
        "DEFINE TABLE alien_with_explicit_attributes;"
    );

    assert_eq!(
        AlienWithExplicitAttributes::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE alien_with_explicit_attributes;
DEFINE FIELD name ON TABLE alien_with_explicit_attributes;
DEFINE FIELD age ON TABLE alien_with_explicit_attributes;
DEFINE FIELD created ON TABLE alien_with_explicit_attributes;
DEFINE FIELD lifeExpectancy ON TABLE alien_with_explicit_attributes;
DEFINE FIELD territoryArea ON TABLE alien_with_explicit_attributes;
DEFINE FIELD home ON TABLE alien_with_explicit_attributes;
DEFINE FIELD tags ON TABLE alien_with_explicit_attributes;
DEFINE FIELD ally ON TABLE alien_with_explicit_attributes TYPE record (alien_with_explicit_attributes);
DEFINE FIELD weapon ON TABLE alien_with_explicit_attributes TYPE record (weapon);
DEFINE FIELD spaceShips ON TABLE alien_with_explicit_attributes TYPE array;
DEFINE FIELD spaceShips.* ON TABLE alien_with_explicit_attributes TYPE record (space_ship);"
    );

    Ok(())
}
