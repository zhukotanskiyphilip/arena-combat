/*
═══════════════════════════════════════════════════════════════════════════════
 МОДУЛЬ: src/physics/mod.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Фізична симуляція для Active Ragdoll системи.
   Базується на принципах Hellish Quart - м'язи (PD-контролери) керують
   фізичними кістками для досягнення цільових поз.

🎯 КОМПОНЕНТИ:
   - PhysicsWorld: обгортка над Rapier3D
   - Skeleton: ієрархія кісток з фізичними тілами
   - Muscle: PD-контролер для керування суглобом
   - ActiveRagdoll: комбінація скелета + м'язів

═══════════════════════════════════════════════════════════════════════════════
*/

pub mod skeleton;
pub mod muscle;
pub mod ragdoll;

pub use skeleton::{Skeleton, Bone, BoneId};
pub use muscle::{Muscle, MuscleSystem};
pub use ragdoll::ActiveRagdoll;

use rapier3d::prelude::*;
pub use rapier3d::prelude::nalgebra;
use glam::{Vec3, Quat};

/// Обгортка над Rapier3D фізичним світом
pub struct PhysicsWorld {
    /// Параметри гравітації
    pub gravity: Vector<f32>,

    /// Rapier components
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,

    /// Integration parameters
    integration_parameters: IntegrationParameters,

    /// Physics pipeline
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    /// Створює новий фізичний світ (оптимізовано для ragdolls)
    pub fn new() -> Self {
        // Оптимізовані параметри для active ragdolls
        let mut integration_parameters = IntegrationParameters::default();
        // Note: Rapier 0.22 uses different solver parameters
        // Solver iterations are configured per-joint via motor parameters
        integration_parameters.dt = 1.0 / 60.0;  // 60 Hz physics

        Self {
            gravity: vector![0.0, -9.81, 0.0],
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    /// Крок фізичної симуляції
    pub fn step(&mut self, delta: f32) {
        self.integration_parameters.dt = delta;

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    /// Додає rigid body і повертає handle
    pub fn add_rigid_body(&mut self, body: RigidBody) -> RigidBodyHandle {
        self.rigid_body_set.insert(body)
    }

    /// Додає collider до rigid body
    pub fn add_collider(&mut self, collider: Collider, parent: RigidBodyHandle) -> ColliderHandle {
        self.collider_set.insert_with_parent(collider, parent, &mut self.rigid_body_set)
    }

    /// Додає joint між двома тілами
    pub fn add_joint(
        &mut self,
        body1: RigidBodyHandle,
        body2: RigidBodyHandle,
        joint: impl Into<GenericJoint>,
    ) -> ImpulseJointHandle {
        self.impulse_joint_set.insert(body1, body2, joint, true)
    }

    /// Отримує позицію rigid body
    pub fn get_body_position(&self, handle: RigidBodyHandle) -> Option<Vec3> {
        self.rigid_body_set.get(handle).map(|body| {
            let pos = body.translation();
            Vec3::new(pos.x, pos.y, pos.z)
        })
    }

    /// Отримує ротацію rigid body
    pub fn get_body_rotation(&self, handle: RigidBodyHandle) -> Option<Quat> {
        self.rigid_body_set.get(handle).map(|body| {
            let rot = body.rotation();
            Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w)
        })
    }

    /// Застосовує torque до rigid body
    pub fn apply_torque(&mut self, handle: RigidBodyHandle, torque: Vec3) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.add_torque(vector![torque.x, torque.y, torque.z], true);
        }
    }

    /// Застосовує force до rigid body
    pub fn apply_force(&mut self, handle: RigidBodyHandle, force: Vec3) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.add_force(vector![force.x, force.y, force.z], true);
        }
    }

    /// Створює землю (статичний collider)
    pub fn create_ground(&mut self, y: f32) {
        let ground = RigidBodyBuilder::fixed()
            .translation(vector![0.0, y, 0.0])
            .build();
        let ground_handle = self.rigid_body_set.insert(ground);

        // Ground має колізуватись з GROUP_1 (кістками скелета)
        // membership: GROUP_2 (ground group)
        // filter: ALL (колізія з усіма)
        let ground_collider = ColliderBuilder::cuboid(50.0, 0.1, 50.0)
            .friction(0.8)
            .restitution(0.0)  // Без відскоку
            .collision_groups(InteractionGroups::new(
                Group::GROUP_2,  // Ground is in GROUP_2
                Group::ALL,      // Collide with everything
            ))
            .build();
        self.collider_set.insert_with_parent(ground_collider, ground_handle, &mut self.rigid_body_set);
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Конвертація glam Vec3 -> Rapier Vector
pub fn vec3_to_rapier(v: Vec3) -> Vector<f32> {
    vector![v.x, v.y, v.z]
}

/// Конвертація Rapier Vector -> glam Vec3
pub fn rapier_to_vec3(v: &Vector<f32>) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

/// Конвертація glam Quat -> Rapier UnitQuaternion
pub fn quat_to_rapier(q: Quat) -> nalgebra::UnitQuaternion<f32> {
    nalgebra::UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(q.w, q.x, q.y, q.z))
}

/// Конвертація Rapier UnitQuaternion -> glam Quat
pub fn rapier_to_quat(q: &nalgebra::UnitQuaternion<f32>) -> Quat {
    Quat::from_xyzw(q.i, q.j, q.k, q.w)
}
