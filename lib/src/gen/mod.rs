// use bevy::math::DVec2;
// use rand::Rng;
// use rand_distr::Distribution;

// use crate::sim::{
//     bundles::StarBundle,
//     components::{CelestialBodySystemId, CelestialBodyId, CelestialBodyName},
//     resources::{CelestialBody, Galaxy},
// };

// pub mod distr;
// pub mod physics;

// pub struct IncompleteCelestialBody {
//     pub pos: DVec2,
//     pub mass: f64,
//     pub repr: String,
// }

// impl IncompleteCelestialBody {
//     pub fn new(pos: DVec2, mass: f64) -> Self {
//         IncompleteCelestialBody {
//             pos,
//             mass,
//             repr: format!("{},{} {:?}", pos.x, pos.y, mass),
//         }
//     }

//     pub fn complete(self, vel: DVec2) -> CelestialBody {
//         CelestialBody::from_incomplete(self, vel)
//     }

//     pub fn get_string_repr(&self) -> &str {
//         &self.repr
//     }
// }

// pub struct IncompleteStarSystem {
//     pub id: usize,
//     pub center: DVec2,
//     pub radius: f64,
//     pub bodies: Vec<CelestialBody>,
// }

// impl IncompleteStarSystem {
//     pub fn new(id: usize, center: DVec2, radius: f64) -> Self {
//         IncompleteStarSystem {
//             id,
//             center,
//             radius,
//             bodies: Vec::new(),
//         }
//     }

//     pub fn add_body(&mut self, body: CelestialBody) -> CelestialBodySystemId {
//         let galactic_id = CelestialBodySystemId {
//             in_system_id: self.bodies.len(),
//             system_id: self.id,
//         };
//         self.bodies.push(body);
//         galactic_id
//     }
// }

// pub struct IncompleteGalaxy {
//     pub body_count: usize,
//     pub star_systems: Vec<IncompleteStarSystem>,
// }

// impl IncompleteGalaxy {
//     // pub fn create_body(&mut self, system_id: usize, rng: &mut impl Rng) -> StarBundle {
//     //     let body = IncompleteCelestialBody::new(
//     //         distr::GalaxyStarDensityDistribution::new(self.star_systems[system_id].radius)
//     //             .sample(rng),
//     //         distr::StarMassDistribution.sample(rng),
//     //     );

//     //     let (id, system_id) = self.add_body(system_id, body.complete(DVec2::ZERO));

//     //     StarBundle {
//     //         id,
//     //         system_id,
//     //         name: CelestialBodyName("".to_string()),
//     //         star_class: crate::sim::components::StarClass::O,
//     //     }
//     // }

//     // pub fn add_system(&mut self, center: DVec2, radius: f64) -> usize {
//     //     let id = self.star_systems.len();
//     //     self.star_systems
//     //         .push(IncompleteStarSystem::new(id, center, radius));
//     //     id
//     // }

//     // pub fn add_body(
//     //     &mut self,
//     //     galaxy_id: usize,
//     //     body: CelestialBody,
//     // ) -> (CelestialBodyId, CelestialBodySystemId) {
//     //     self.body_count += 1;
//     //     (
//     //         CelestialBodyId(self.body_count),
//     //         self.star_systems[galaxy_id].add_body(body),
//     //     )
//     // }

//     // pub fn complete(self) -> Galaxy {
//     //     self.into()
//     // }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_star_gen() {
//         let mut rng = rand::thread_rng();
//         let mut galaxy = IncompleteGalaxy {
//             body_count: 0,
//             star_systems: Vec::new(),
//         };
//         let system_id = galaxy.add_system(DVec2::ZERO, 100.);
//         for _ in 0..1000000 {
//             let bundle = galaxy.create_body(system_id, &mut rng);
//             let body = galaxy.star_systems[bundle.system_id.system_id].bodies[bundle.system_id.in_system_id];
//             if body.mass() / 1.9891e30 > 10. {
//                 println!("{:?} {} {}", body.mass() / 1.9891e30, body.pos(), body.vel());
//             }
//         }
//     }
// }
