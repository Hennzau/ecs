use ecs::core::component::{
    ComponentBuilder,
    AnyComponent,
};

#[derive(Clone, ComponentBuilder)]
pub struct Position2Df32 {
    pub x: f32,
    pub y: f32,
}

impl Position2Df32 {
    pub fn new(x: f32, y: f32) -> Self {
        return Self {
            x,
            y,
        };
    }

    pub fn origin() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Position2Di32 {
    pub x: i32,
    pub y: i32,
}

impl Position2Di32 {
    pub fn new(x: i32, y: i32) -> Self {
        return Self {
            x,
            y,
        };
    }

    pub fn origin() -> Self {
        return Self {
            x: 0,
            y: 0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Position3Df32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3Df32 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self {
            x,
            y,
            z,
        };
    }

    pub fn origin() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Position3Di32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position3Di32 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        return Self {
            x,
            y,
            z,
        };
    }

    pub fn origin() -> Self {
        return Self {
            x: 0,
            y: 0,
            z: 0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Velocity2Df32 {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity2Df32 {
    pub fn new(dx: f32, dy: f32) -> Self {
        return Self {
            dx,
            dy,
        };
    }

    pub fn zero() -> Self {
        return Self {
            dx: 0.0,
            dy: 0.0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Velocity2Di32 {
    pub dx: i32,
    pub dy: i32,
}

impl Velocity2Di32 {
    pub fn new(dx: i32, dy: i32) -> Self {
        return Self {
            dx,
            dy,
        };
    }

    pub fn zero() -> Self {
        return Self {
            dx: 0,
            dy: 0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Velocity3Df32 {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}

impl Velocity3Df32 {
    pub fn new(dx: f32, dy: f32, dz: f32) -> Self {
        return Self {
            dx,
            dy,
            dz,
        };
    }

    pub fn zero() -> Self {
        return Self {
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Velocity3Di32 {
    pub dx: i32,
    pub dy: i32,
    pub dz: i32,
}

impl Velocity3Di32 {
    pub fn new(dx: i32, dy: i32, dz: i32) -> Self {
        return Self {
            dx,
            dy,
            dz,
        };
    }

    pub fn zero() -> Self {
        return Self {
            dx: 0,
            dy: 0,
            dz: 0,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Scale2Df32 {
    pub width: f32,
    pub height: f32,
}

impl Scale2Df32 {
    pub fn new(width: f32, height: f32) -> Self {
        return Self {
            width,
            height,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Scale2Di32 {
    pub width: i32,
    pub height: i32,
}

impl Scale2Di32 {
    pub fn new(width: i32, height: i32) -> Self {
        return Self {
            width,
            height,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Scale3Df32 {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl Scale3Df32 {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        return Self {
            width,
            height,
            depth,
        };
    }
}

#[derive(Clone, ComponentBuilder)]
pub struct Scale3Di32 {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

impl Scale3Di32 {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        return Self {
            width,
            height,
            depth,
        };
    }
}

impl std::ops::Add for Position2Df32 {
    type Output = Position2Df32;

    fn add(self, other: Position2Df32) -> Position2Df32 {
        return Position2Df32 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Add for Position3Df32 {
    type Output = Position3Df32;

    fn add(self, other: Position3Df32) -> Position3Df32 {
        return Position3Df32 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl std::ops::AddAssign for Position2Df32 {
    fn add_assign(&mut self, other: Position2Df32) {
        *self = Position2Df32 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::AddAssign for Position3Df32 {
    fn add_assign(&mut self, other: Position3Df32) {
        *self = Position3Df32 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.y + other.z,
        };
    }
}

impl std::ops::Mul<f32> for Velocity2Df32 {
    type Output = Position2Df32;

    fn mul(self, time: f32) -> Position2Df32 {
        return Position2Df32 {
            x: self.dx * time,
            y: self.dy * time,
        };
    }
}

impl std::ops::Mul<f32> for Velocity3Df32 {
    type Output = Position3Df32;

    fn mul(self, time: f32) -> Position3Df32 {
        return Position3Df32 {
            x: self.dx * time,
            y: self.dy * time,
            z: self.dz * time,
        };
    }
}