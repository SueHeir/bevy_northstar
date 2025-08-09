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

#[cfg(feature = "gui-debug")]
use crate::debug::DebugTilemapType;

/// An entities position on the pathfinding [`crate::grid::Grid`].
/// You'll need to maintain this position if you use the plugin pathfinding systems.
#[derive(Component, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct AgentPos(pub UVec3);

/****************************************
    PATHFINDING COMPONENTS
*****************************************/

/// Determines which algorithm to use for pathfinding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
pub enum PathfindMode {
    /// Hierarchical pathfinding with the final path refined with line tracing.
    #[default]
    Refined,
    /// Hierarchical pathfinding using only cached paths. Use this if you're not concerned with trying to find the shortest path.
    Coarse,
    /// Full-grid A* pathfinding without hierarchy.
    /// Useful for small grids or a turn based pathfinding path where movement cost needs to be the most accurate and cpu usage isn't a concern.
    AStar,
}

/// Insert [`Pathfind`] on an entity to pathfind to a goal.
/// Once the plugin systems have found a path, [`NextPos`] will be inserted.
#[derive(Component, Default, Debug, Reflect)]
pub struct Pathfind {
    /// The goal to pathfind to.
    pub goal: UVec3,
    /// Will attempt to return the best path if full route isn't found.
    pub partial: bool,

    /// The [`PathfindMode`] to use for pathfinding.
    /// Defaults to [`PathfindMode::Refined`] which is hierarchical pathfinding with full refinement.
    pub mode: PathfindMode,
}

impl Pathfind {
    /// Creates a new [`Pathfind`] component with the given goal.
    /// An HPA* refined path will be returned by default.
    /// If you want to use a different pathfinding mode, use the [`Pathfind::mode()`] method.
    /// If you want to allow partial paths, use the [`Pathfind::partial()`] method.
    /// # Example
    /// ```rust,no_run
    /// use bevy::math::UVec3;
    /// use bevy_northstar::prelude::*;
    ///
    /// let pathfind = Pathfind::new(UVec3::new(5, 5, 0))
    ///     .mode(PathfindMode::AStar)
    ///     .partial();
    /// ```
    ///
    pub fn new(goal: UVec3) -> Self {
        Pathfind {
            goal,
            ..Default::default()
        }
    }

    /// Shorthand constructor for 2D pathfinding to avoid needing to construct a [`bevy::math::UVec3`].
    /// This will set the z-coordinate to 0.
    pub fn new_2d(x: u32, y: u32) -> Self {
        Pathfind {
            goal: UVec3::new(x, y, 0),
            ..Default::default()
        }
    }

    /// Shorthand constructor for 3D pathfinding to avoid needing to construct a [`bevy::math::UVec3`].
    pub fn new_3d(x: u32, y: u32, z: u32) -> Self {
        Pathfind {
            goal: UVec3::new(x, y, z),
            ..Default::default()
        }
    }

    /// Sets the pathfinding mode. See [`PathfindMode`] for options.
    pub fn mode(mut self, mode: PathfindMode) -> Self {
        self.mode = mode;
        self
    }

    /// Allow partial paths.
    /// The pathfinding system will return the best path it can find
    /// even if it can't find a full route to the goal.
    pub fn partial(mut self) -> Self {
        self.partial = true;
        self
    }
}

/// The next position in the path inserted into an entity by the pathfinding system.
/// The `pathfind` system in [`crate::plugin::NorthstarPlugin`] will insert this.
/// Remove [`NextPos`] after you've moved the entity to the next position and
/// a new [`NextPos`] will be inserted on the next frame.
#[derive(Component, Default, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct NextPos(pub UVec3);

// See src/path.rs for the Path component

/****************************************
    COLLISION COMPONENTS
*****************************************/

/// Marker component for entities that dynamically block paths during navigation.
///
/// The pathfinding system’s collision avoidance checks for entities with this component
/// to treat their positions as temporarily blocked.
///
/// **Do not** use this component for static obstacles such as walls or terrain.
/// Static geometry should be handled separately with [`crate::grid::Grid::set_nav()`] in [`crate::grid::Grid`].
#[derive(Component, Default)]
pub struct Blocking;

// I want to switch to this in the future on the next Bevy major release.
/*#[derive(Component, Debug)]
pub enum PathError {
    /// Unable to find a path to the goal.
    NoPathFound,
    /// The next position in the path is now impassable due to dynamic changes to the grid.
    PathInvalidated,
    /// The pathfinding system failed to reroute the entity around an obstacle with `Blocking`.
    /// `NorthstarPlugin` reroute_path system will attempt to deeper reroute. You can also handle this yourself by running your system before [`crate::prelude::PathingSet`].
    AvoidanceFailed,
    /// The pathfinding system failed to reroute the entity to its goal after all avoidance options were exhausted.
    /// This means the entity cannot reach its goal and you will need to handle this failure in your own system.
    /// Examples would be to set a new goal or wait for a certain amount of time before trying to reroute again.
    /// **You will need to handle this failure in your own system before the entity can be pathed again**.
    RerouteFailed,
}

impl PartialEq for PathError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PathError::NoPathFound, PathError::NoPathFound)
                | (PathError::PathInvalidated, PathError::PathInvalidated)
                | (PathError::AvoidanceFailed, PathError::AvoidanceFailed)
                | (PathError::RerouteFailed, PathError::RerouteFailed)
        )
    }
}*/

/// Marker component that is inserted on an entity when local avoidance fails.
/// Currently this marker is handled by the [`crate::plugin::NorthstarPlugin`] `reroute_path` system and can be ignored
/// unless the desire is to handle the failure in a custom way.
#[derive(Component, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct AvoidanceFailed;

/// Marker component that is inserted on an entity when a collision is detected.
/// The built-in pathfinding system will try to pathfind for this entity every frame unless
/// you handle the failure in a custom way.
#[derive(Component, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct PathfindingFailed;

/// Marker component that is inserted on an entity when path rerouting in [`crate::plugin::NorthstarPlugin`] `reroute_path` fails.
/// This happens well all avoidance options have been exhausted and the entity cannot be rerouted to its goal.
/// **You will need to handle this failure in your own system before the entity can be pathed again**.
/// Examples would be to set a new goal or wait for a certain amount of time before trying to reroute again.
#[derive(Component, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct RerouteFailed;

/****************************************
    DEBUGGING COMPONENTS
*****************************************/

#[cfg(feature = "gui-debug")]
pub mod debug_components;

/****************************************
    GRID RELATIONSHIPS
*****************************************/

/// The [`AgentOfGrid`] component is used to create a relationship between an agent or entity and the grid it belongs to.
/// Pass your [`crate::grid::Grid`] entity to this component and insert it on your entity to relate it so all
/// pathfinding systems and debugging know which grid to use.
#[derive(Component, Reflect)]
#[relationship(relationship_target = GridAgents)]
pub struct AgentOfGrid(pub Entity);

/// The [`GridAgents`] component is used to store a list of entities that are agents in a grid.
/// See [`AgentOfGrid`] for more information on how to associate an entity with a grid.
#[derive(Component, Reflect)]
#[relationship_target(relationship = AgentOfGrid, linked_spawn)]
pub struct GridAgents(Vec<Entity>);

impl GridAgents {
    /// Returns all the entities that have a relationship with the grid.
    pub fn entities(&self) -> &[Entity] {
        &self.0
    }
}
