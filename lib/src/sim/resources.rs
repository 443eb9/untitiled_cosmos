use std::collections::VecDeque;

use bevy::{ecs::system::Resource, math::DVec2, render::color::Color};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::consts;

use super::components::{CelestialBodyId, CelestialBodySystemId};

#[cfg(feature = "debug")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

#[derive(Resource)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Resource))]
pub struct SimulationTimeScale(pub u32);

impl Default for SimulationTimeScale {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Debug, Reflect))]
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

#[derive(Clone)]
pub struct Orbit {
    verts: VecDeque<DVec2>,
    pub color: Color,
}

impl Orbit {
    pub fn new(iterations: usize, color: Color) -> Self {
        Orbit {
            verts: vec![DVec2::ZERO; iterations].into(),
            color,
        }
    }

    #[inline]
    pub fn push(&mut self, pos: DVec2) {
        self.verts.push_back(pos);
    }

    #[inline]
    pub fn pop(&mut self) -> DVec2 {
        self.verts.pop_front().unwrap()
    }

    #[inline]
    pub fn update(&mut self, pos: DVec2) -> DVec2 {
        self.verts.push_back(pos);
        self.verts.pop_front().unwrap()
    }

    #[inline]
    pub fn vertices(&self) -> &VecDeque<DVec2> {
        &self.verts
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub struct StarSystem {
    pub id: usize,
    pub bodies: Vec<CelestialBodyId>,
}

impl StarSystem {
    pub fn new(id: usize) -> Self {
        StarSystem {
            id,
            bodies: Vec::new(),
        }
    }

    #[inline]
    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn add_body(&mut self, body: CelestialBodyId) -> CelestialBodySystemId {
        let in_system_id = self.bodies.len();
        self.bodies.push(body);
        CelestialBodySystemId {
            in_system_id,
            system_id: self.id,
        }
    }
}

#[derive(Resource)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Resource))]
pub struct Galaxy {
    time_step: f64,
    star_systems: Vec<StarSystem>,
    bodies: Vec<CelestialBody>,
    body_colors: Vec<Color>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            time_step: consts::FIXED_STEP,
            star_systems: Default::default(),
            bodies: Default::default(),
            body_colors: Default::default(),
        }
    }
}

impl Galaxy {
    #[inline]
    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    #[inline]
    pub fn system_count(&self) -> usize {
        self.star_systems.len()
    }

    pub fn add_body(
        &mut self,
        system_id: usize,
        body: CelestialBody,
    ) -> (CelestialBodyId, CelestialBodySystemId) {
        if system_id >= self.star_systems.len() {
            (0..system_id - self.star_systems.len() + 1).for_each(|_| self.create_system());
        }

        let id = CelestialBodyId(self.body_count());
        let sys_id = self.star_systems[system_id].add_body(id);
        self.bodies.push(body);
        (id, sys_id)
    }

    pub fn create_system(&mut self) {
        self.star_systems
            .push(StarSystem::new(self.star_systems.len()));
    }

    #[inline]
    pub fn set_color(&mut self, id: CelestialBodyId, color: Color) {
        if id.0 >= self.body_colors.len() {
            self.body_colors.resize(id.0 + 1, Color::WHITE);
        }
        self.body_colors[id.0] = color;
    }

    #[inline]
    pub fn get_body(&self, id: CelestialBodyId) -> Option<&CelestialBody> {
        self.bodies.get(id.0)
    }

    #[inline]
    pub fn systems(&self) -> &[StarSystem] {
        &self.star_systems
    }

    #[inline]
    pub fn step(&mut self) {
        calc_acc(&mut self.bodies);
        update_pos(&mut self.bodies, self.time_step);
    }
}

#[derive(Resource, Default)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Resource))]
pub struct OrbitPredictor {
    iterations: usize,
    time_step: f64,
    #[cfg_attr(feature = "debug", reflect(ignore))]
    parallel_universe: Vec<CelestialBody>,
    #[cfg_attr(feature = "debug", reflect(ignore))]
    orbits: Vec<Orbit>,
}

impl OrbitPredictor {
    #[inline]
    pub fn iterations(&self) -> usize {
        self.iterations
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Orbit> {
        self.orbits.iter()
    }

    #[inline]
    pub fn get_orbit(&self, id: CelestialBodyId) -> Option<&Orbit> {
        self.orbits.get(id.0)
    }

    pub fn update_state(&mut self, iterations: usize, galaxy: &Galaxy) {
        self.iterations = iterations;
        self.parallel_universe = galaxy.bodies.clone();
        self.time_step = consts::FIXED_STEP;
        self.orbits = galaxy
            .body_colors
            .iter()
            .map(|color| Orbit::new(iterations, *color))
            .collect();
        for _ in 0..iterations {
            self.step()
        }
    }

    pub fn step(&mut self) {
        if self.iterations != 0 {
            calc_acc(&mut self.parallel_universe);
            update_pos(&mut self.parallel_universe, self.time_step);
            self.parallel_universe
                .par_iter()
                .zip(self.orbits.par_iter_mut())
                .for_each(|(body, orbit)| {
                    orbit.update(body.pos);
                });
        }
    }
}

fn calc_acc(bodies: &mut [CelestialBody]) {
    bodies.par_iter_mut().for_each(|body| {
        body.acc = DVec2::ZERO;
    });

    for i_lhs in 0..bodies.len() {
        let lhs = bodies[i_lhs];
        bodies.par_iter_mut().enumerate().for_each(|(i_rhs, rhs)| {
            if i_lhs == i_rhs {
                return;
            }
            let dist = (rhs.pos - lhs.pos).length_squared();
            rhs.acc += consts::G * lhs.mass / dist * (lhs.pos - rhs.pos).normalize();
        });
    }
}

fn update_pos(bodies: &mut [CelestialBody], dt: f64) {
    bodies.par_iter_mut().for_each(|body| {
        body.vel += body.acc * dt;
        body.pos += body.vel * dt;
    });
}
