use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct LastMoveTime(pub f32);

#[derive(Resource, InspectorOptions)]
pub struct Grid {
    pub size: Vec2,
    pub cell_size: Vec2,
}

impl Grid {
    pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        IVec2::new(
            (world_pos.x / self.cell_size.x).floor() as i32,
            (world_pos.y / self.cell_size.y).floor() as i32,
        )
    }

    pub fn grid_to_world(&self, grid_pos: IVec2) -> Vec2 {
        Vec2::new(
            grid_pos.x as f32 * self.cell_size.x + self.cell_size.x / 2.0,
            grid_pos.y as f32 * self.cell_size.y + self.cell_size.y / 2.0,
        )
    }
}


#[derive(Component, InspectorOptions)]
pub struct Player;
