use bevy::{ecs::system::Resource, math::DVec2};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::consts;

use super::components::{CelestialBodyId, CelestialBodySystemId};

#[cfg(feature = "debug")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

#[derive(Resource, Clone, Copy)]
pub struct TimeScale(pub f64);

impl Default for TimeScale {
    fn default() -> Self {
        Self(0.02)
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub struct CelestialBody {
    pos: DVec2,
    radius: f64,
    mass: f64,
    vel: DVec2,
    acc: DVec2,
}

impl CelestialBody {
    pub fn new(pos: DVec2, radius: f64, mass: f64, vel: DVec2) -> Self {
        CelestialBody {
            pos,
            radius,
            mass,
            vel,
            acc: DVec2::ZERO,
        }
    }

    #[inline]
    pub fn pos(&self) -> DVec2 {
        self.pos
    }

    #[inline]
    pub fn mass(&self) -> f64 {
        self.mass
    }

    #[inline]
    pub fn vel(&self) -> DVec2 {
        self.vel
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub struct StarSystem {
    id: usize,
    center: DVec2,
    radius: f64,
    bodies: Vec<CelestialBody>,
}

impl StarSystem {
    pub fn new(id: usize, center: DVec2, radius: f64) -> Self {
        StarSystem {
            id,
            center,
            radius,
            bodies: Vec::new(),
        }
    }

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

    pub fn add_body(&mut self, body: CelestialBody) -> CelestialBodySystemId {
        let in_system_id = self.bodies.len();
        self.bodies.push(body);
        CelestialBodySystemId {
            in_system_id,
            system_id: self.id,
        }
    }

    pub fn get_body(&self, id: CelestialBodySystemId) -> Option<&CelestialBody> {
        self.bodies.get(id.in_system_id)
    }
}

#[derive(Resource, Default)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[reflect(Resource)]
pub struct Galaxy {
    body_count: usize,
    star_systems: Vec<StarSystem>,
}

impl Galaxy {
    #[inline]
    pub fn body_count(&self) -> usize {
        self.body_count
    }

    #[inline]
    pub fn system_count(&self) -> usize {
        self.star_systems.len()
    }

    #[inline]
    pub fn add_body(
        &mut self,
        system_id: usize,
        body: CelestialBody,
    ) -> (CelestialBodyId, CelestialBodySystemId) {
        let sys_id = self.star_systems[system_id].add_body(body);
        self.body_count += 1;
        (CelestialBodyId(self.body_count - 1), sys_id)
    }

    #[inline]
    pub fn add_system(&mut self, system: StarSystem) {
        self.star_systems.push(system);
    }

    #[inline]
    pub fn get_body(&self, id: CelestialBodySystemId) -> Option<&CelestialBody> {
        self.star_systems
            .get(id.system_id)
            .and_then(|sys| sys.get_body(id))
    }

    pub fn calc_acc(&mut self) {
        self.star_systems.par_iter_mut().for_each(|galaxy| {
            galaxy.bodies.par_iter_mut().for_each(|body| {
                body.acc = DVec2::ZERO;
            });
        });

        self.star_systems.par_iter_mut().for_each(|galaxy| {
            for i_lhs in 0..galaxy.bodies.len() {
                let lhs = galaxy.bodies[i_lhs];
                galaxy
                    .bodies
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(i_rhs, rhs)| {
                        if i_lhs == i_rhs {
                            return;
                        }
                        let dist = (rhs.pos - lhs.pos).length_squared();
                        rhs.acc += consts::G * rhs.mass / dist * (lhs.pos - rhs.pos).normalize();
                    });
            }
        });
    }

    pub fn update_pos(&mut self, dt: f64) {
        self.star_systems.par_iter_mut().for_each(|galaxy| {
            galaxy.bodies.par_iter_mut().for_each(|body| {
                body.vel += body.acc * dt;
                body.pos += body.vel * dt;
            });
        });
    }
}
