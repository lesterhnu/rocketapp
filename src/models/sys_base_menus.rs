//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_base_menus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub menu_level: Option<u64>,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub hidden: Option<i8>,
    pub component: Option<String>,
    pub sort: Option<i64>,
    pub keep_alive: Option<i8>,
    pub default_menu: Option<i8>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub close_tab: Option<i8>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
