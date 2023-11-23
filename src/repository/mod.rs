use crate::repository::entity::Entity;

mod entity;
mod sqlite;

pub trait Repository<C, E: Entity, Err> {
    fn connect_to_db(connection_string: &str) -> Result<C, Err>;
    fn create_db(&self) -> Result<(), Err>;
    fn save_new_item(&self, item: &E::ItemDto) -> Result<E::Id, Err>;
    fn select_item_by_id(&self, id: &E::Id) -> Result<E::Item, Err>;
    fn update_item(&self, item: &E::Item) -> Result<(), Err>;
    fn delete_item_by_id(&self, id: &E::Id) -> Result<(), Err>;
}
