use crate::ecs::entity::EntityId;
use ahash::AHashMap;
use std::any::{Any, TypeId};

pub struct World {
    entities: Vec<EntityId>,
    components: AHashMap<TypeId, Box<dyn ComponentStorage>>,
    next_entity_id: u32,
    free_list: Vec<EntityId>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: AHashMap::new(),
            next_entity_id: 0,
            free_list: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = if let Some(recycled) = self.free_list.pop() {
            recycled
        } else {
            let new_id = EntityId(self.next_entity_id);
            self.next_entity_id += 1;
            new_id
        };

        self.entities.push(id);
        id
    }

    pub fn destroy_entity(&mut self, entity: EntityId) {
        if let Some(pos) = self.entities.iter().position(|&e| e == entity) {
            self.entities.swap_remove(pos);
            self.free_list.push(entity);

            for storage in self.components.values_mut() {
                storage.remove(entity);
            }
        }
    }

    pub fn add_component<T: Component>(&mut self, entity: EntityId, component: T) {
        let type_id = TypeId::of::<T>();

        if !self.components.contains_key(&type_id) {
            self.components
                .insert(type_id, Box::new(ComponentVec::<T>::new()));
        }

        if let Some(storage) = self.components.get_mut(&type_id) {
            if let Some(typed_storage) = storage.as_any_mut().downcast_mut::<ComponentVec<T>>() {
                typed_storage.insert(entity, component);
            }
        }
    }

    pub fn get_component<T: Component>(&self, entity: EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        self.components
            .get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentVec<T>>()?
            .get(entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        self.components
            .get_mut(&type_id)?
            .as_any_mut()
            .downcast_mut::<ComponentVec<T>>()?
            .get_mut(entity)
    }

    pub fn query<T: Component>(&self) -> impl Iterator<Item = (EntityId, &T)> {
        let type_id = TypeId::of::<T>();

        self.components
            .get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<ComponentVec<T>>())
            .map(|typed_storage| typed_storage.iter())
            .into_iter()
            .flatten()
    }

    pub fn query_mut<T: Component>(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        let type_id = TypeId::of::<T>();

        self.components
            .get_mut(&type_id)
            .and_then(|storage| storage.as_any_mut().downcast_mut::<ComponentVec<T>>())
            .map(|typed_storage| typed_storage.iter_mut())
            .into_iter()
            .flatten()
    }
}

pub trait Component: 'static + Send + Sync {}

trait ComponentStorage: Any + Send + Sync {
    fn remove(&mut self, entity: EntityId);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct ComponentVec<T: Component> {
    data: AHashMap<EntityId, T>,
}

impl<T: Component> ComponentVec<T> {
    fn new() -> Self {
        Self {
            data: AHashMap::new(),
        }
    }

    fn insert(&mut self, entity: EntityId, component: T) {
        self.data.insert(entity, component);
    }

    fn get(&self, entity: EntityId) -> Option<&T> {
        self.data.get(&entity)
    }

    fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.data.iter().map(|(&id, comp)| (id, comp))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.data.iter_mut().map(|(id, comp)| (*id, comp))
    }
}

impl<T: Component> ComponentStorage for ComponentVec<T> {
    fn remove(&mut self, entity: EntityId) {
        self.data.remove(&entity);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
