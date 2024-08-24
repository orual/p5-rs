use bevy::prelude::Component;
use bevy::{color::Color, math::IVec2};
use std::ops::{Deref, DerefMut};
use std::{fmt::Debug, hash::Hash};

pub trait Cell: Clone + Component {
    type Coords: Clone + Debug + Send + Sync + Eq + Hash;

    /// Retrieves the cell coordinates
    #[must_use]
    fn coords(&self) -> &Self::Coords;

    // Retrieves the coordinates of the neighbour cells
    fn neighbour_coords(&self) -> impl IntoIterator<Item = Self::Coords>;
}

#[derive(Debug, Clone, Component, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Cell2d(pub IVec2);

const NEIGHBOUR_COORDS_MOORE: [IVec2; 8] = [
    // Left
    IVec2::new(-1, 0),
    // Top Left
    IVec2::new(-1, 1),
    // Top
    IVec2::new(0, 1),
    // Top Right
    IVec2::new(1, 1),
    // Right
    IVec2::new(1, 0),
    // Bottom Right
    IVec2::new(1, -1),
    // Bottom
    IVec2::new(0, -1),
    // Bottom Left
    IVec2::new(-1, -1),
];

impl Cell for Cell2d {
    type Coords = IVec2;

    #[inline]
    fn coords(&self) -> &Self::Coords {
        &self.0
    }

    #[inline]
    fn neighbour_coords(&self) -> impl IntoIterator<Item = Self::Coords> {
        NEIGHBOUR_COORDS_MOORE.map(|c| c + *self.coords())
    }
}

impl Deref for Cell2d {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Pixel: Component + Sized + Clone + PartialEq {
    fn color(&self) -> Color;
}

#[derive(Debug, Clone, Component, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct BwPixel(pub bool);

impl Pixel for BwPixel {
    #[inline]
    fn color(&self) -> Color {
        if self.0 {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

impl Deref for BwPixel {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BwPixel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<bool> for BwPixel {
    fn from(val: bool) -> Self {
        Self(val)
    }
}

#[derive(Debug, Clone, Component, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct GrayPixel(pub f32);

impl Pixel for GrayPixel {
    #[inline]
    fn color(&self) -> Color {
        Color::oklch(self.0, 0.0, 0.0)
    }
}

impl Deref for GrayPixel {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GrayPixel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<f32> for GrayPixel {
    fn from(val: f32) -> Self {
        Self(val)
    }
}

#[derive(Debug, Clone, Component, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ColorPixel(pub Color);

impl Pixel for ColorPixel {
    #[inline]
    fn color(&self) -> Color {
        self.0
    }
}

impl Deref for ColorPixel {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ColorPixel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Color> for ColorPixel {
    fn from(val: Color) -> Self {
        Self(val)
    }
}
