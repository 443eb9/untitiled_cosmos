use std::collections::VecDeque;

use bevy::{ecs::system::Resource, math::DVec2, render::color::Color, utils::HashSet};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::consts;

use super::components::CelestialBodyId;

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

#[derive(Default, Clone, Copy)]
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

#[derive(Resource)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Resource))]
pub struct Galaxy {
    time_step: f64,
    bodies: Vec<CelestialBody>,
    body_colors: Vec<Color>,
    body_id_to_index: Vec<Option<usize>>,
    body_index_to_id: Vec<Option<CelestialBodyId>>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            time_step: consts::CELESTIAL_SIM_STEP,
            bodies: Default::default(),
            body_colors: Default::default(),
            body_id_to_index: Default::default(),
            body_index_to_id: Default::default(),
        }
    }
}

impl<'a> IntoParallelRefIterator<'a> for Galaxy {
    type Item = &'a CelestialBody;
    type Iter = rayon::slice::Iter<'a, CelestialBody>;

    fn par_iter(&'a self) -> Self::Iter {
        self.bodies.par_iter()
    }
}

impl Galaxy {
    #[inline]
    pub fn num_bodies(&self) -> usize {
        self.bodies.len()
    }

    pub fn add_body(&mut self, body: CelestialBody) -> CelestialBodyId {
        let id = CelestialBodyId(self.num_bodies());
        self.body_id_to_index.push(Some(self.bodies.len()));
        self.body_index_to_id.push(Some(id));
        self.bodies.push(body);
        id
    }

    pub fn remove_body(&mut self, id: CelestialBodyId) {
        if let Some(index) = self.body_id_to_index[id.0] {
            for i in index + 1..self.body_id_to_index.len() {
                if let Some(idx) = &mut self.body_id_to_index[i] {
                    *idx -= 1;
                }
            }

            for i in index..self.body_index_to_id.len() - 1 {
                if let Some(id) = &mut self.body_index_to_id[i] {
                    *id = CelestialBodyId(id.0 + 1);
                }
            }
            self.body_index_to_id.pop();
            self.body_id_to_index[id.0] = None;
            self.bodies.remove(index);
        }
    }

    #[inline]
    pub fn bodies(&self) -> &[CelestialBody] {
        &self.bodies
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
        self.body_id_to_index
            .get(id.0)
            .map(|&i| i.map(|i| &self.bodies[i]))
            .flatten()
    }

    #[inline]
    pub fn step(&mut self) {
        calc_acc(&mut self.bodies);
        update_pos(&mut self.bodies, self.time_step);
    }

    #[inline]
    pub fn test_overlapping(&self) -> HashSet<CelestialBodyId> {
        let mut overlapping = HashSet::default();
        self.bodies
            .iter()
            .enumerate()
            .for_each(|(i_lhs, body_lhs)| {
                self.bodies
                    .iter()
                    .enumerate()
                    .for_each(|(i_rhs, body_rhs)| {
                        if i_lhs == i_rhs {
                            return;
                        }

                        if (body_lhs.pos - body_rhs.pos).length_squared()
                            > (body_lhs.radius + body_rhs.radius).powi(2)
                        {
                            return;
                        }

                        let remove = {
                            if body_lhs.mass > body_rhs.mass {
                                i_rhs
                            } else {
                                i_lhs
                            }
                        };
                        overlapping.insert(self.body_index_to_id[remove].unwrap());
                    });
            });
        overlapping
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
        self.time_step = consts::CELESTIAL_SIM_STEP;
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
