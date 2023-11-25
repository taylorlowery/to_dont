use crate::repository::entity::Entity;

mod entity;
pub mod sqlite;

/// The `Repository` trait defines a set of common CRUD operations.
///
/// The
pub trait Repository<C, E: Entity, Err> {
    fn connect_to_db(connection_string: &str) -> Result<C, Err>;
    fn save_new_item(&self, item: &E::ItemDto) -> Result<E::Id, Err>;
    fn select_item_by_id(&self, id: &E::Id) -> Result<E::Item, Err>;
    fn update_item(&self, id: &E::Id, item: &E::ItemDto) -> Result<usize, Err>;
    fn delete_item_by_id(&self, id: &E::Id) -> Result<usize, Err>;
}
