use bevy::{ecs::system::Resource, math::DVec2};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    consts,
    gen::{IncompleteCelestialBody, IncompleteGalaxy, IncompleteUniverse},
};

#[derive(Resource, Clone, Copy)]
pub struct TimeScale(pub f64);

impl Default for TimeScale {
    fn default() -> Self {
        Self(0.02)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CelestialBody {
    pub pos: DVec2,
    pub mass: f64,
    pub vel: DVec2,
    pub acc: DVec2,
}

impl CelestialBody {
    pub fn from_incomplete(incomplete: IncompleteCelestialBody, vel: DVec2) -> Self {
        CelestialBody {
            pos: incomplete.pos,
            mass: incomplete.mass,
            vel,
            acc: DVec2::ZERO,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Galaxy {
    id: usize,
    center: DVec2,
    radius: f64,
    bodies: Vec<CelestialBody>,
}

impl From<IncompleteGalaxy> for Galaxy {
    fn from(value: IncompleteGalaxy) -> Self {
        Galaxy {
            id: value.id,
            center: value.center,
            radius: value.radius,
            bodies: value.bodies,
        }
    }
}

impl Galaxy {
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn center(&self) -> DVec2 {
        self.center
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    #[inline]
    pub fn bodies(&self) -> &[CelestialBody] {
        &self.bodies
    }
}

#[derive(Resource, Default)]
pub struct Universe {
    body_count: usize,
    galaxies: Vec<Galaxy>,
}

impl From<IncompleteUniverse> for Universe {
    fn from(value: IncompleteUniverse) -> Self {
        Universe {
            body_count: value.body_count,
            galaxies: value.galaxies.into_iter().map(|g| g.into()).collect(),
        }
    }
}

impl Universe {
    #[inline]
    pub fn body_count(&self) -> usize {
        self.body_count
    }
    
    pub fn calc_acc(&mut self) {
        self.galaxies.par_iter_mut().for_each(|galaxy| {
            galaxy.bodies.par_iter_mut().for_each(|body| {
                body.acc = DVec2::ZERO;
            });
        });

        self.galaxies.par_iter_mut().for_each(|galaxy| {
            for i in 0..galaxy.bodies.len() {
                let lhs = galaxy.bodies[i];
                galaxy.bodies.par_iter_mut().for_each(|rhs| {
                    let dist = (rhs.pos - lhs.pos).length_squared();
                    rhs.acc += consts::G * rhs.mass / dist * (lhs.pos - rhs.pos).normalize();
                });
            }
        });
    }

    pub fn update_pos(&mut self, dt: f64) {
        self.galaxies.par_iter_mut().for_each(|galaxy| {
            galaxy.bodies.par_iter_mut().for_each(|body| {
                body.vel += body.acc * dt;
                body.pos += body.vel * dt;
            });
        });
    }
}
