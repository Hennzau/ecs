use ecs::prelude::*;

#[derive(Component)]
pub struct Position2Df32 {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Position2Di32 {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Position2Du32 {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Position3Df32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Position3Di32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct Position3Du32 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Component)]
pub struct Velocity2Df32 {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity2Di32 {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Velocity2Du32 {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Velocity3Df32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Velocity3Di32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct Scale2Df32 {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Scale2Du32 {
    pub x: u32,
    pub y: u32,
}