extern crate rayon;
extern crate specs;
use crate::constant;
use crate::laser::intensity_gradient::LaserIntensityGradientSamplers;
use specs::{Join, ReadStorage, System, WriteStorage};
extern crate nalgebra;
use crate::atom::Force;
use crate::dipole::atom::AtomicDipoleTransition;
use crate::laser::dipole_beam::{DipoleLight, DipoleLightIndex};
use nalgebra::Vector3;

/// System that calculates the forces exerted onto the atoms by the dipole laser beams
/// It uses the `LaserIntensityGradientSamplers` and the properties of the `DipoleLight`
/// to add the respective amount of force to `Force`
pub struct ApplyDipoleForceSystem;

impl<'a> System<'a> for ApplyDipoleForceSystem {
    type SystemData = (
        ReadStorage<'a, DipoleLight>,
        ReadStorage<'a, DipoleLightIndex>,
        ReadStorage<'a, AtomicDipoleTransition>,
        ReadStorage<'a, LaserIntensityGradientSamplers>,
        WriteStorage<'a, Force>,
    );

    fn run(
        &mut self,
        (dipole_light, dipole_index,atomic_transition, gradient_sampler, mut force): Self::SystemData,
    ) {
        use rayon::prelude::ParallelIterator;
        use specs::ParJoin;
        (&mut force, &atomic_transition, &gradient_sampler)
            .par_join()
            .for_each(|(mut force, atominfo, sampler)| {
                let prefactor = -3. * constant::PI * constant::C.powf(2.0)
                    / (2. * (2. * constant::PI * atominfo.frequency).powf(3.0))
                    * atominfo.linewidth;
                let mut temp_force_coeff = Vector3::new(0.0, 0.0, 0.0);
                for (index, dipole) in (&dipole_index, &dipole_light).join() {
                    temp_force_coeff = temp_force_coeff
                        - (1. / (atominfo.frequency - dipole.frequency())
                            + 1. / (atominfo.frequency + dipole.frequency()))
                            * sampler.contents[index.index].gradient;
                }
                force.force = force.force + prefactor * temp_force_coeff;
            });
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    extern crate specs;
    use assert_approx_eq::assert_approx_eq;
    use specs::{Builder, RunNow, World};
    extern crate nalgebra;
    use crate::laser;
    use crate::laser::gaussian::GaussianBeam;
    use nalgebra::Vector3;

    #[test]
    fn test_apply_dipole_force_system() {
        let mut test_world = World::new();

        test_world.register::<DipoleLightIndex>();
        test_world.register::<DipoleLight>();
        test_world.register::<Force>();
        test_world.register::<LaserIntensityGradientSamplers>();
        test_world.register::<AtomicDipoleTransition>();

        test_world
            .create_entity()
            .with(DipoleLightIndex {
                index: 0,
                initiated: true,
            })
            .with(DipoleLight {
                wavelength: 1064.0e-9,
            })
            .build();

        let transition = AtomicDipoleTransition::strontium();
        let atom1 = test_world
            .create_entity()
            .with(Force {
                force: Vector3::new(0.0, 0.0, 0.0),
            })
            .with(LaserIntensityGradientSamplers {
                contents: [crate::laser::intensity_gradient::LaserIntensityGradientSampler {
                    gradient: Vector3::new(0.0, 1.0, -2.0),
                }; crate::dipole::BEAM_LIMIT],
            })
            .with(transition)
            .build();
        let mut system = ApplyDipoleForceSystem;
        system.run_now(&test_world.res);
        test_world.maintain();
        let sampler_storage = test_world.read_storage::<Force>();
        let sim_result_force = sampler_storage.get(atom1).expect("Entity not found!").force;

        let actual_force = 3. * constant::PI * constant::C.powf(2.0)
            / (2. * (2. * constant::PI * transition.frequency).powf(3.0))
            * transition.linewidth
            * (1. / (transition.frequency - 1064.0e-9) + 1. / (transition.frequency + 1064.0e-9))
            * Vector3::new(0.0, 1.0, -2.0);

        assert_approx_eq!(actual_force[0], sim_result_force[0], 1e+8_f64);
        assert_approx_eq!(actual_force[1], sim_result_force[1], 1e+8_f64);
        assert_approx_eq!(actual_force[2], sim_result_force[2], 1e+8_f64);
    }

    #[test]
    fn test_apply_dipole_force_again_system() {
        let mut test_world = World::new();

        test_world.register::<DipoleLightIndex>();
        test_world.register::<DipoleLight>();
        test_world.register::<Force>();
        test_world.register::<LaserIntensityGradientSamplers>();
        test_world.register::<AtomicDipoleTransition>();

        test_world
            .create_entity()
            .with(DipoleLightIndex {
                index: 0,
                initiated: true,
            })
            .with(DipoleLight {
                wavelength: 1064.0e-9,
            })
            .build();

        let transition = AtomicDipoleTransition::strontium();
        let atom1 = test_world
            .create_entity()
            .with(Force {
                force: Vector3::new(0.0, 0.0, 0.0),
            })
            .with(LaserIntensityGradientSamplers {
                contents: [crate::laser::intensity_gradient::LaserIntensityGradientSampler {
                    gradient: Vector3::new(-8.4628e+7, -4.33992902e+13, -4.33992902e+13),
                }; crate::dipole::BEAM_LIMIT],
            })
            .with(transition)
            .build();
        let mut system = ApplyDipoleForceSystem;
        system.run_now(&test_world.res);
        test_world.maintain();
        let sampler_storage = test_world.read_storage::<Force>();
        let sim_result_force = sampler_storage.get(atom1).expect("Entity not found!").force;

        assert_approx_eq!(-6.06743188e-29, sim_result_force[0], 3e-30_f64);
        assert_approx_eq!(-3.11151847e-23, sim_result_force[1], 2e-24_f64);
        assert_approx_eq!(-3.11151847e-23, sim_result_force[2], 2e-24_f64);
    }

    #[test]
    fn test_apply_dipole_force_and_gradient_system() {
        let mut test_world = World::new();

        test_world.register::<DipoleLightIndex>();
        test_world.register::<DipoleLight>();
        test_world.register::<Force>();
        test_world.register::<LaserIntensityGradientSamplers>();
        test_world.register::<AtomicDipoleTransition>();
        test_world.register::<crate::atom::Position>();
        test_world.register::<crate::laser::gaussian::GaussianBeam>();
        test_world.register::<crate::laser::gaussian::GaussianReferenceFrame>();

        let power = 10.0;
        let e_radius = 60.0e-6 / (2.0_f64.sqrt());

        let gaussian_beam = GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: e_radius,
            power: power,
            direction: Vector3::x(),
            rayleigh_range: crate::laser::gaussian::calculate_rayleigh_range(&1064.0e-9, &e_radius),
        };
        test_world
            .create_entity()
            .with(gaussian_beam)
            .with(laser::dipole_beam::DipoleLight {
                wavelength: 1064.0e-9,
            })
            .with(DipoleLightIndex {
                index: 0,
                initiated: true,
            })
            .with(laser::gaussian::GaussianReferenceFrame {
                x_vector: Vector3::y(),
                y_vector: Vector3::z(),
                ellipticity: 0.0,
            })
            .build();
        let gaussian_beam = GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: e_radius,
            power: power,
            direction: Vector3::y(),
            rayleigh_range: crate::laser::gaussian::calculate_rayleigh_range(&1064.0e-9, &e_radius),
        };
        test_world
            .create_entity()
            .with(gaussian_beam)
            .with(laser::dipole_beam::DipoleLight {
                wavelength: 1064.0e-9,
            })
            .with(DipoleLightIndex {
                index: 1,
                initiated: true,
            })
            .with(laser::gaussian::GaussianReferenceFrame {
                x_vector: Vector3::x(),
                y_vector: Vector3::z(),
                ellipticity: 0.0,
            })
            .build();

        let transition = AtomicDipoleTransition::strontium();
        let atom1 = test_world
            .create_entity()
            .with(crate::atom::Position {
                pos: Vector3::new(-1.0e-4, -1.0e-4, -2.0e-4),
            })
            .with(Force {
                force: Vector3::new(0.0, 0.0, 0.0),
            })
            .with(LaserIntensityGradientSamplers {
                contents: [laser::intensity_gradient::LaserIntensityGradientSampler::default();
                    crate::dipole::BEAM_LIMIT],
            })
            .with(transition)
            .build();
        let mut grad_system = laser::intensity_gradient::SampleLaserIntensityGradientSystem;
        let mut force_system = ApplyDipoleForceSystem;
        grad_system.run_now(&test_world.res);
        test_world.maintain();
        force_system.run_now(&test_world.res);
        test_world.maintain();
        let sampler_storage = test_world.read_storage::<Force>();
        let grad_sampler_storage = test_world.read_storage::<LaserIntensityGradientSamplers>();
        let sim_result_force = sampler_storage.get(atom1).expect("Entity not found!").force;
        let _sim_result_grad = grad_sampler_storage
            .get(atom1)
            .expect("Entity not found!")
            .contents;
        //println!("force is: {}", sim_result_force);
        //println!("gradient 1 is: {}", sim_result_grad[0].gradient);
        //println!("gradient 2 is: {}", sim_result_grad[1].gradient);

        assert_approx_eq!(
            0.00000000000000000000000000000000012747566586448897,
            sim_result_force[0],
            3e-46_f64
        );
        assert_approx_eq!(
            0.00000000000000000000000000000000012747566586448897,
            sim_result_force[1],
            2e-46_f64
        );
        assert_approx_eq!(
            0.0000000000000000000000000000000005101243283409891,
            sim_result_force[2],
            2e-46_f64
        );
    }
}
