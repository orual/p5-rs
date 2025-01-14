use bevy::{
    prelude::{Entity, Resource},
    utils::{HashMap, HashSet},
};

use super::cells::{Cell, Cell2d};

/// A `CellMap` implementation for `Cell2d`
pub type Map2D = CellMap<Cell2d>;

/// Global Cell container resource , uses a `Hashmap`to allow non-continuous
/// cells.
///
/// The resource is automatically added and refreshed, it may be used for
/// clearing (see examples).
#[derive(Clone, Resource)]
pub struct CellMap<C: Cell> {
    cells: HashMap<C::Coords, Entity>,
}

impl<C: Cell> Default for CellMap<C> {
    fn default() -> Self {
        Self {
            cells: Default::default(),
        }
    }
}

impl<C: Cell> CellMap<C> {
    /// Retrieves every cell entity matching `coords`.
    /// If some coordinates are not stored in the cell map they will be ignored.
    pub fn get_cell_entities<'a>(
        &'a self,
        coords: &'a [C::Coords],
    ) -> impl Iterator<Item = &Entity> + 'a {
        coords.iter().filter_map(|c| self.cells.get(c))
    }

    /// Adds a `Cell` entity to the map at `coordinates`.
    ///
    /// # Note:
    ///
    /// This operation is done automatically when you add a `Cell` component to
    /// an entity.
    ///
    /// # Returns
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned
    pub fn insert_cell(&mut self, coordinates: C::Coords, entity: Entity) -> Option<Entity> {
        self.cells.insert(coordinates, entity)
    }

    /// Removes a cell from the map, returning the `Entity` value if it was
    /// present.
    ///
    /// # Note:
    ///
    /// Use this method to remove cell entities from the map if you remove a
    /// `Cell` component from an `Entity` or *despawn* an `Entity` with a
    /// `Cell` component.
    pub fn remove_cell(&mut self, coordinates: &C::Coords) -> Option<Entity> {
        self.cells.remove(coordinates)
    }

    /// Removes a cell entities from the map
    pub fn remove_entities(&mut self, entities: impl Iterator<Item = Entity>) {
        let entities: HashSet<_> = entities.collect();
        if entities.is_empty() {
            return;
        }
        self.cells.retain(|_, entity| !entities.contains(entity));
    }

    /// Retrieves a cell entity using its `coordinates`
    pub fn get_cell(&self, coordinates: &C::Coords) -> Option<Entity> {
        self.cells.get(coordinates).copied()
    }

    /// Clears the entire map
    pub fn clear(&mut self) {
        self.cells.clear();
    }
}
