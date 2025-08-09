//! Components for pathfinding, collision, and debugging.
use bevy::{
    color::palettes::css,
    ecs::entity::Entity,
    math::{UVec3, Vec2, Vec3},
    platform::collections::HashMap,
    prelude::{Color, Component},
    reflect::Reflect,
    transform::components::Transform,
};

use crate::debug::DebugTilemapType;

/****************************************
    DEBUGGING COMPONENTS
*****************************************/

/// Add this component to the same entity as [`DebugPath`] to offset the debug gizmos.
/// Useful for aligning the gizmos with your tilemap rendering offset.
#[derive(Component, Default, Reflect)]
pub struct DebugOffset(pub Vec3);

/// You can add DebugDepthOffsets to your DebugGrid entity and the debug gizmo's y position
/// will be offset by the depth (z-coordinate) of the grid/path position.
#[derive(Component, Default, Reflect)]
pub struct DebugDepthYOffsets(pub HashMap<u32, f32>);

/// Add [`DebugCursor`] to your DebugGrid entity and provide it with the current position
/// of your mouse cursor.
/// This will allow [`DebugGrid::set_show_connections_on_hover()`] to only draw connections graph node under the cursor.
#[derive(Component, Debug, Default, Reflect)]
pub struct DebugCursor(pub Option<Vec2>);

// Internal component to hold which cell the mouse is hovering over.
#[derive(Component, Debug, Default)]
pub(crate) struct DebugNode(pub(crate) Option<UVec3>);

/// Component for debugging an entity's [`crate::path::Path`].
#[derive(Component, Reflect)]
pub struct DebugPath {
    /// The [`Color`] of the path gizmo.
    pub color: Color,
    /// Draw the HPA* high level graph path between chunk entrances.
    /// This is useful for debugging the HPA* algorithm.
    pub draw_unrefined: bool,
}

impl DebugPath {
    /// Creates a new [`DebugPath`] component with the specified color.
    /// The default color is red.
    pub fn new(color: Color) -> Self {
        DebugPath {
            color,
            draw_unrefined: false,
        }
    }
}

impl Default for DebugPath {
    fn default() -> Self {
        DebugPath {
            color: bevy::prelude::Color::Srgba(css::RED),
            draw_unrefined: false,
        }
    }
}

/// Component for debugging [`crate::grid::Grid`].
/// You need to insert [`DebugGrid`] as a child of your map.
#[derive(Reflect, Component)]
#[require(Transform, DebugOffset, DebugDepthYOffsets, DebugCursor, DebugNode)]
pub struct DebugGrid {
    /// The width of your tiles in pixels.
    pub tile_width: u32,
    /// The height of your tiles in pixels.
    pub tile_height: u32,
    /// The depth of your 3D grid.
    pub depth: u32,
    /// The type of tilemap being used.
    pub map_type: DebugTilemapType,
    /// Will outline the chunks that the grid is divided into.
    pub draw_chunks: bool,
    /// Will draw the [`crate::nav::NavCell`]s in your grid.
    pub draw_cells: bool,
    /// Will draw the HPA* graph entrance nodes in each chunk.
    pub draw_entrances: bool,
    /// Will draw the internal cached paths between the entrances.
    pub draw_cached_paths: bool,
    /// Will show the connections between nodes only when hovering over them.
    pub show_connections_on_hover: bool,
}

impl DebugGrid {
    /// The width and height of a tile in pixels. This is required because your tile pixel dimensions may not match the grid size.
    pub fn tile_size(&mut self, width: u32, height: u32) -> &Self {
        self.tile_width = width;
        self.tile_height = height;
        self
    }

    /// Sets the z depth to draw for 3d tilemaps.
    pub fn set_depth(&mut self, depth: u32) -> &Self {
        self.depth = depth;
        self
    }

    /// Gets the z depth that the debug grid is drawing for 3D tilemaps.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Set the [`DebugTilemapType`] which is used to determine how the grid is visualized (e.g., square or isometric). Align this with the style of your tilemap.
    pub fn map_type(&mut self, map_type: DebugTilemapType) -> &Self {
        self.map_type = map_type;
        self
    }

    /// Will outline the chunks that the grid is divided into.
    pub fn set_draw_chunks(&mut self, value: bool) -> &Self {
        self.draw_chunks = value;
        self
    }

    /// Toggle draw_chunks.
    pub fn toggle_chunks(&mut self) -> &Self {
        self.draw_chunks = !self.draw_chunks;
        self
    }

    /// Will draw the [`crate::nav::NavCell`]s in your grid. This should align to each tile in your tilemap.
    pub fn set_draw_cells(&mut self, value: bool) -> &Self {
        self.draw_cells = value;
        self
    }

    /// Toggle draw_cells.
    pub fn toggle_cells(&mut self) -> &Self {
        self.draw_cells = !self.draw_cells;
        self
    }

    /// The entrances are the cells in each chunk that connect to other chunks.
    /// This will draw the entrances calculated by the HPA* algorithm.
    /// This is very useful for debugging the HPA* algorithm and understanding how chunks are connected to build the hierarchy.
    pub fn set_draw_entrances(&mut self, value: bool) -> &Self {
        self.draw_entrances = value;
        self
    }

    /// Toggle draw_entrances.
    pub fn toggle_entrances(&mut self) -> &Self {
        self.draw_entrances = !self.draw_entrances;
        self
    }

    /// Draws the internal cached paths between the entrances in the same chunk.
    /// This is only really useful for debugging odd issues with the HPA* crate.
    pub fn set_draw_cached_paths(&mut self, value: bool) -> &Self {
        self.draw_cached_paths = value;
        self
    }

    /// Toggle draw_cached_paths.
    pub fn toggle_cached_paths(&mut self) -> &Self {
        self.draw_cached_paths = !self.draw_cached_paths;
        self
    }

    /// Settings this to true will ONLY draw connections (edges, cached_paths) for entrances that are under the mouse cursor.
    /// This is useful to get a clearer view of the HPA* connections without other entrances paths overlapping.
    /// You will need to manually update [`DebugCursor`] to the UVec3 tile/cell your mouse is over.
    pub fn set_show_connections_on_hover(&mut self, value: bool) -> &Self {
        self.show_connections_on_hover = value;
        self
    }

    /// Toggle show_connections_on_hover.
    pub fn toggle_show_connections_on_hover(&mut self) -> &Self {
        self.show_connections_on_hover = !self.show_connections_on_hover;
        self
    }
}

/// Builder for [`DebugGrid`].
/// Use this to configure debugging for a grid before inserting it into your map entity.
/// Insert the returned [`DebugGrid`] as a child of the entity with your [`crate::grid::Grid`] component.
pub struct DebugGridBuilder {
    tile_width: u32,
    tile_height: u32,
    depth: u32,
    tilemap_type: DebugTilemapType,
    draw_chunks: bool,
    draw_cells: bool,
    draw_entrances: bool,
    draw_cached_paths: bool,
    show_connections_on_hover: bool,
}

impl DebugGridBuilder {
    /// Creates a new [`DebugGridBuilder`] with the specified tile width and height.
    pub fn new(tile_width: u32, tile_height: u32) -> Self {
        Self {
            tile_width,
            tile_height,
            depth: 0,
            tilemap_type: DebugTilemapType::Square,
            draw_chunks: false,
            draw_cells: false,
            draw_entrances: false,
            draw_cached_paths: false,
            show_connections_on_hover: false,
        }
    }

    /// Sets which z depth the debug grid will draw for 3D tilemaps.
    pub fn set_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    /// Sets the draw type of the tilemap.
    /// This is used to determine how the grid is visualized (e.g., square or isometric).
    /// Use the shorthand methods [`DebugGridBuilder::isometric()`] to set this instead.
    pub fn tilemap_type(mut self, tilemap_type: DebugTilemapType) -> Self {
        self.tilemap_type = tilemap_type;
        self
    }

    /// Utility function to set the [`DebugGrid`] to draw in isometric.
    pub fn isometric(mut self) -> Self {
        self.tilemap_type = DebugTilemapType::Isometric;
        self
    }

    /// Enables drawing the outline of chunks the grid is divided into.
    pub fn enable_chunks(mut self) -> Self {
        self.draw_chunks = true;
        self
    }

    /// Enables drawing the [`crate::nav::NavCell`]s in your grid.
    /// Useful for visualizing the navigation movement options in your grid and how they align with your tilemap.
    pub fn enable_cells(mut self) -> Self {
        self.draw_cells = true;
        self
    }

    /// Enables drawing the chunk entrances calculated by the HPA* algorithm.
    /// This is useful for debugging how chunks are connected and how the HPA* algorithm builds its hierarchy.
    pub fn enable_entrances(mut self) -> Self {
        self.draw_entrances = true;
        self
    }

    /// Enables drawing the cached paths between entrances in the same chunk.
    /// Is only useful for debugging odd issues with the HPA* crate.
    pub fn enable_cached_paths(mut self) -> Self {
        self.draw_cached_paths = true;
        self
    }

    /// Enables drawing connections (edges, cached_paths) only for the entrance under the mouse cursor.
    /// This is useful to get a clearer view of the HPA* connections without other entrances paths overlapping.
    /// You will need to manually update [`DebugCursor`] to the UVec3 tile/cell your mouse is over.
    pub fn enable_show_connections_on_hover(mut self) -> Self {
        self.show_connections_on_hover = true;
        self
    }

    /// Builds the final [`DebugGrid`] component with the configured settings to be inserted into your map entity.
    /// You need to call this methdod to finalize the builder and create the component.
    pub fn build(self) -> DebugGrid {
        DebugGrid {
            tile_width: self.tile_width,
            tile_height: self.tile_height,
            depth: self.depth,
            map_type: self.tilemap_type,
            draw_chunks: self.draw_chunks,
            draw_cells: self.draw_cells,
            draw_entrances: self.draw_entrances,
            draw_cached_paths: self.draw_cached_paths,
            show_connections_on_hover: self.show_connections_on_hover,
        }
    }
}
