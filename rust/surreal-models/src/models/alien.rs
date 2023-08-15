use std::time::Duration;

use crate::{AlienVisitsPlanet, Planet, Rocket, SpaceShip, Weapon};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{LinkMany, LinkOne, LinkSelf, Node, Relate, SurrealSimpleId};
use surrealdb::sql;

// Alien
#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: geo::LineString,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,
    pub tags: Vec<String>,
    // database type attribute is autogenerated for all links of the struct. But you can also provide it
    #[surreal_orm(link_self = "Alien")]
    pub ally: LinkSelf<Alien>,

    #[surreal_orm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    #[surreal_orm(
        link_many = "SpaceShip",
        // type = "array",
        // content_type = "record(space_ship)"
    )]
    pub space_ships: LinkMany<SpaceShip>,

    // This is a read only field
    #[surreal_orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet>,
}

// Explicityly specifying all field types. Most of it can be inferred.
// So, you usually wouldn't have to annotate the type manually. (See Alien).
// Adding this for testing purpose.
#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "alien_with_explicit_attributes")]
pub struct AlienWithExplicitAttributes {
    #[surreal_orm(type = "record(alien_with_explicit_attributes)")]
    id: sql::Thing,

    #[surreal_orm(type = "string")]
    name: String,

    #[surreal_orm(type = "int")]
    age: u8,

    #[surreal_orm(type = "datetime")]
    created: DateTime<Utc>,

    #[surreal_orm(type = "duration")]
    life_expectancy: Duration,

    #[surreal_orm(type = "geometry(feature)")]
    territory_area: geo::Polygon,

    #[surreal_orm(type = "geometry(feature)")]
    home: geo::Point,

    #[surreal_orm(content_type = "string")]
    // Full definitions. This also works.
    // #[surreal_orm(type = "array", content_type = "string")]
    tags: Vec<String>,

    // database type attribute is autogenerated for all links of the struct. But you can also provide it
    #[surreal_orm(
        link_self = "AlienWithExplicitAttributes",
        type = "record(alien_with_explicit_attributes)"
    )]
    ally: LinkSelf<AlienWithExplicitAttributes>,

    #[surreal_orm(link_one = "Weapon", type = "record(weapon)")]
    weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    #[surreal_orm(
        link_many = "SpaceShip",
        type = "array",
        content_type = "record(space_ship)"
    )]
    space_ships: LinkMany<SpaceShip>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "alien_2")]
pub struct Alien2 {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: geo::LineString,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,
    pub tags: Vec<String>,

    #[surreal_orm(nest_object = "Rocket")]
    pub weapon: Rocket,

    // Again, we dont have to provide the type attribute, it can auto detect
    #[surreal_orm(
        link_many = "SpaceShip",
        type = "array",
        content_type = "record(space_ship)"
    )]
    pub space_ships: LinkMany<SpaceShip>,
}
