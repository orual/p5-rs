pub mod cells;
pub mod map;

use bevy::{log, prelude::*, utils::HashMap};
use cells::{Cell, Cell2d, Pixel};
use map::CellMap;

use bevy::prelude::Resource;

/// Resource to insert for parallel queries and batching
#[derive(Debug, Clone, Resource, Default)]
pub struct SimulationBatch;

/// Resource to insert to pause the cellular automaton simulation
#[derive(Debug, Resource)]
pub struct SimulationPause;

fn handle_pixel<C, P>((cell, value): (&C, &P), map: &HashMap<C::Coords, P>) -> Option<P>
where
    C: Cell,
    P: Pixel,
{
    match map.get(cell.coords()) {
        Some(pixel) if value == pixel => None,
        _ => Some(value.clone()),
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_pixels<C, P>(
    mut commands: Commands,
    par_commands: ParallelCommands,
    query: Query<(Entity, &C, &P)>,
    pause: Option<Res<SimulationPause>>,
    batch: Option<Res<SimulationBatch>>,
) where
    C: Cell,
    P: Pixel,
{
    if pause.is_some() {
        return;
    }
    let map: HashMap<<C as Cell>::Coords, P> = query
        .iter()
        .map(|(_entity, cell, value)| (cell.coords().clone(), value.clone()))
        .collect();
    if batch.is_some() {
        query.par_iter().for_each(|(entity, cell, value)| {
            if let Some(new_value) = handle_pixel((cell, value), &map) {
                par_commands.command_scope(|mut cmd| {
                    cmd.entity(entity).try_insert(new_value);
                });
            }
        });
    } else {
        for (entity, cell, value) in query.iter() {
            if let Some(new_value) = handle_pixel((cell, value), &map) {
                commands.entity(entity).try_insert(new_value);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_new_pixels<C>(query: Query<(Entity, &C), Added<C>>, mut map: ResMut<CellMap<C>>)
where
    C: Cell,
{
    for (entity, new_cell) in query.iter() {
        let old_entity = map.insert_cell(new_cell.coords().clone(), entity);
        if let Some(e) = old_entity {
            if e != entity {
                log::warn!(
                    "{:?} replaced {:?} at {:?} coordinates",
                    entity,
                    e,
                    new_cell.coords()
                );
            }
        }
    }
}

pub fn handle_removed_pixels<C>(
    mut removed_cells: RemovedComponents<C>,
    mut map: ResMut<CellMap<C>>,
) where
    C: Cell,
{
    if removed_cells.is_empty() {
        return;
    }
    log::trace!("Removing {} cells from cell map", removed_cells.len());
    map.remove_entities(removed_cells.read());
}

pub fn px_to_sprite<'a, P>(
    (cell, value): (&'a Cell2d, &'a P),
    size: f32,
) -> (SpriteBundle, &'a Cell2d)
where
    P: Pixel,
{
    (
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(size)),
                color: value.color(),
                ..default()
            },
            transform: Transform::from_xyz(
                size * cell.coords().x as f32,
                size * cell.coords().y as f32,
                0.,
            ),
            ..default()
        },
        cell,
    )
}

#[inline]
fn apply_color<P>(value: &P, visible: &mut Visibility, sprite: &mut Sprite)
where
    P: Pixel,
{
    sprite.color = value.color();
    if *visible != Visibility::Inherited {
        *visible = Visibility::Inherited;
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn color_sprites<P>(
    mut query: Query<(&P, &mut Visibility, &mut Sprite), Changed<P>>,
    batch: Option<Res<SimulationBatch>>,
) where
    P: Pixel,
{
    if batch.is_some() {
        query
            .par_iter_mut()
            .for_each(|(state, mut visible, mut sprite)| {
                apply_color(state, &mut visible, &mut sprite);
            });
    } else {
        for (state, mut visible, mut sprite) in &mut query {
            apply_color(state, &mut visible, &mut sprite);
        }
    }
}
