//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_operation_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub ip: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i64>,
    pub latency: Option<i64>,
    pub agent: Option<String>,
    pub error_message: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub body: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub resp: Option<String>,
    pub user_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}