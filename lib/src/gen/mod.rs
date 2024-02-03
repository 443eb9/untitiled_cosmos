use bevy::math::DVec2;
use rand::Rng;
use rand_distr::Distribution;

use crate::{
    nn,
    sim::{
        bundles::StarBundle,
        components::{CelestialBodyGalacticId, CelestialBodyId, CelestialBodyName},
        resources::{CelestialBody, Universe},
    },
};

pub mod distr;

pub struct IncompleteCelestialBody {
    pub pos: DVec2,
    pub mass: f64,
    pub repr: String,
}

impl IncompleteCelestialBody {
    pub fn new(pos: DVec2, mass: f64) -> Self {
        IncompleteCelestialBody {
            pos,
            mass,
            repr: format!("{},{} {:?}", pos.x, pos.y, mass),
        }
    }

    pub fn complete(self, vel: DVec2) -> CelestialBody {
        CelestialBody::from_incomplete(self, vel)
    }

    pub fn get_string_repr(&self) -> &str {
        &self.repr
    }
}

pub struct IncompleteGalaxy {
    pub id: usize,
    pub center: DVec2,
    pub radius: f64,
    pub bodies: Vec<CelestialBody>,
    pub repr: String,
}

impl IncompleteGalaxy {
    pub fn new(id: usize, center: DVec2, radius: f64) -> Self {
        IncompleteGalaxy {
            id,
            center,
            radius,
            bodies: Vec::new(),
            repr: format!("{},{} {}|", center.x, center.y, radius),
        }
    }

    pub fn get_string_repr(&self) -> &str {
        &self.repr
    }

    pub fn add_body(
        &mut self,
        body: IncompleteCelestialBody,
        vel: DVec2,
    ) -> CelestialBodyGalacticId {
        let galactic_id = CelestialBodyGalacticId {
            id: self.bodies.len(),
            galaxy_id: self.id,
        };
        self.repr
            .push_str(&format!("{} {},{}|", body.get_string_repr(), vel.x, vel.y));
        self.bodies.push(body.complete(vel));
        galactic_id
    }
}

pub struct IncompleteUniverse {
    pub body_count: usize,
    pub galaxies: Vec<IncompleteGalaxy>,
}

impl IncompleteUniverse {
    pub fn add_galaxy(&mut self, center: DVec2, radius: f64) -> usize {
        let id = self.galaxies.len();
        self.galaxies
            .push(IncompleteGalaxy::new(id, center, radius));
        id
    }

    pub fn add_body(
        &mut self,
        galaxy_id: usize,
        body: IncompleteCelestialBody,
        vel: DVec2,
    ) -> (CelestialBodyId, CelestialBodyGalacticId) {
        self.body_count += 1;
        (
            CelestialBodyId(self.body_count),
            self.galaxies[galaxy_id].add_body(body, vel),
        )
    }

    pub fn complete(self) -> Universe {
        self.into()
    }
}

pub struct StarBuilder;

impl StarBuilder {
    pub fn new_to_univ(
        universe: &mut IncompleteUniverse,
        galaxy_id: usize,
        rng: &mut impl Rng,
        seed: u64,
    ) -> StarBundle {
        let body = IncompleteCelestialBody::new(
            distr::GalaxyStarDensityDistribution::new(universe.galaxies[galaxy_id].radius)
                .sample(rng),
            distr::StarMassDistribution.sample(rng),
        );

        let vel = nn::inference_initial_vel(
            seed,
            universe.galaxies[galaxy_id].get_string_repr(),
            body.get_string_repr(),
        );

        let (id, galactic_id) = universe.add_body(galaxy_id, body, vel);

        StarBundle {
            id,
            galactic_id,
            name: CelestialBodyName("".to_string()),
            star_class: crate::sim::components::StarClass::O,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_star_gen() {
        let mut rng = rand::thread_rng();
        let mut universe = IncompleteUniverse {
            body_count: 0,
            galaxies: Vec::new(),
        };
        let galaxy_id = universe.add_galaxy(DVec2::ZERO, 100.);
        for _ in 0..1000000 {
            let bundle = StarBuilder::new_to_univ(&mut universe, galaxy_id, &mut rng, 0);
            let body =
                universe.galaxies[bundle.galactic_id.galaxy_id].bodies[bundle.galactic_id.id];
            if body.mass / 1.9891e30 > 10. {
                println!("{:?} {} {}", body.mass / 1.9891e30, body.pos, body.vel);
            }
        }
    }
}
