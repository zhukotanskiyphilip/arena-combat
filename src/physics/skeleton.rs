/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/physics/skeleton.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Система скелета - ієрархія кісток з фізичними тілами та joints.
   Кожна кістка має:
   - RigidBody (динамічне фізичне тіло)
   - Collider (капсула для колізій)
   - Joint до батьківської кістки (з обмеженнями кутів)

═══════════════════════════════════════════════════════════════════════════════
*/

use rapier3d::prelude::*;
use rapier3d::prelude::nalgebra;
use rapier3d::prelude::nalgebra::UnitQuaternion;
use glam::{Vec3, Quat};
use std::collections::HashMap;

use super::PhysicsWorld;

/// Ідентифікатор кістки (оптимізовано: 11 кісток)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoneId {
    // Торс (3 кістки)
    Pelvis,      // Root
    Spine,       // Single spine body (merged chest + spine)
    Head,        // Head (merged neck)

    // Ліва рука (2 кістки)
    LeftUpperArm,
    LeftLowerArm,

    // Права рука (2 кістки)
    RightUpperArm,
    RightLowerArm,

    // Ліва нога (2 кістки)
    LeftUpperLeg,
    LeftLowerLeg,

    // Права нога (2 кістки)
    RightUpperLeg,
    RightLowerLeg,
}

impl BoneId {
    /// Повертає батьківську кістку (None для root)
    pub fn parent(&self) -> Option<BoneId> {
        match self {
            BoneId::Pelvis => None,
            BoneId::Spine => Some(BoneId::Pelvis),
            BoneId::Head => Some(BoneId::Spine),

            BoneId::LeftUpperArm => Some(BoneId::Spine),
            BoneId::LeftLowerArm => Some(BoneId::LeftUpperArm),

            BoneId::RightUpperArm => Some(BoneId::Spine),
            BoneId::RightLowerArm => Some(BoneId::RightUpperArm),

            BoneId::LeftUpperLeg => Some(BoneId::Pelvis),
            BoneId::LeftLowerLeg => Some(BoneId::LeftUpperLeg),

            BoneId::RightUpperLeg => Some(BoneId::Pelvis),
            BoneId::RightLowerLeg => Some(BoneId::RightUpperLeg),
        }
    }

    /// Список всіх кісток в порядку створення (батьки перед дітьми)
    pub fn all_bones() -> Vec<BoneId> {
        vec![
            BoneId::Pelvis,
            BoneId::Spine,
            BoneId::Head,
            BoneId::LeftUpperArm,
            BoneId::LeftLowerArm,
            BoneId::RightUpperArm,
            BoneId::RightLowerArm,
            BoneId::LeftUpperLeg,
            BoneId::LeftLowerLeg,
            BoneId::RightUpperLeg,
            BoneId::RightLowerLeg,
        ]
    }
}

/// Дані кістки
#[derive(Debug, Clone)]
pub struct Bone {
    pub id: BoneId,
    pub length: f32,
    pub radius: f32,
    pub mass: f32,

    /// Local offset від батьківської кістки (в local space батька)
    pub local_offset: Vec3,

    /// Обмеження кутів суглоба (min, max) для кожної осі
    pub angle_limits: AngleLimits,
}

/// Обмеження кутів суглоба
#[derive(Debug, Clone, Copy)]
pub struct AngleLimits {
    /// Twist (обертання навколо осі кістки)
    pub twist_min: f32,
    pub twist_max: f32,

    /// Swing X (нахил вперед/назад)
    pub swing_x_min: f32,
    pub swing_x_max: f32,

    /// Swing Z (нахил вліво/вправо)
    pub swing_z_min: f32,
    pub swing_z_max: f32,
}

impl Default for AngleLimits {
    fn default() -> Self {
        Self {
            twist_min: -0.5,
            twist_max: 0.5,
            swing_x_min: -0.8,
            swing_x_max: 0.8,
            swing_z_min: -0.8,
            swing_z_max: 0.8,
        }
    }
}

impl AngleLimits {
    /// Без обмежень
    pub fn free() -> Self {
        Self {
            twist_min: -std::f32::consts::PI,
            twist_max: std::f32::consts::PI,
            swing_x_min: -std::f32::consts::PI,
            swing_x_max: std::f32::consts::PI,
            swing_z_min: -std::f32::consts::PI,
            swing_z_max: std::f32::consts::PI,
        }
    }

    /// Коліно (тільки згинання в одній площині)
    pub fn knee() -> Self {
        Self {
            twist_min: -0.1,
            twist_max: 0.1,
            swing_x_min: 0.0,       // Не згинається назад
            swing_x_max: 2.5,       // Згинається вперед ~140°
            swing_z_min: -0.1,
            swing_z_max: 0.1,
        }
    }

    /// Лікоть
    pub fn elbow() -> Self {
        Self {
            twist_min: -0.2,
            twist_max: 0.2,
            swing_x_min: 0.0,
            swing_x_max: 2.4,       // ~135°
            swing_z_min: -0.1,
            swing_z_max: 0.1,
        }
    }

    /// Плече (велика свобода руху)
    pub fn shoulder() -> Self {
        Self {
            twist_min: -1.5,
            twist_max: 1.5,
            swing_x_min: -2.0,
            swing_x_max: 1.0,
            swing_z_min: -0.5,
            swing_z_max: 2.5,
        }
    }

    /// Стегно
    pub fn hip() -> Self {
        Self {
            twist_min: -0.8,
            twist_max: 0.8,
            swing_x_min: -1.5,      // Нога назад
            swing_x_max: 2.0,       // Нога вперед
            swing_z_min: -0.8,      // Нога всередину
            swing_z_max: 1.2,       // Нога назовні
        }
    }

    /// Шия
    pub fn neck() -> Self {
        Self {
            twist_min: -1.0,
            twist_max: 1.0,
            swing_x_min: -0.8,
            swing_x_max: 0.5,
            swing_z_min: -0.6,
            swing_z_max: 0.6,
        }
    }

    /// Хребет (обмежена рухливість)
    pub fn spine() -> Self {
        Self {
            twist_min: -0.4,
            twist_max: 0.4,
            swing_x_min: -0.3,
            swing_x_max: 0.4,
            swing_z_min: -0.3,
            swing_z_max: 0.3,
        }
    }

    /// Зап'ясток/гомілковостопний (середня рухливість)
    pub fn wrist_ankle() -> Self {
        Self {
            twist_min: -0.5,
            twist_max: 0.5,
            swing_x_min: -0.6,
            swing_x_max: 0.8,
            swing_z_min: -0.4,
            swing_z_max: 0.4,
        }
    }
}

/// Фізичний скелет
pub struct Skeleton {
    /// Rigid body handles для кожної кістки
    pub bodies: HashMap<BoneId, RigidBodyHandle>,

    /// Joint handles
    pub joints: HashMap<BoneId, ImpulseJointHandle>,

    /// Дані кісток
    pub bones: HashMap<BoneId, Bone>,

    /// Базова позиція скелета (pelvis)
    pub root_position: Vec3,
}

impl Skeleton {
    /// Створює гуманоїдний скелет
    pub fn create_humanoid(physics: &mut PhysicsWorld, position: Vec3) -> Self {
        let mut skeleton = Self {
            bodies: HashMap::new(),
            joints: HashMap::new(),
            bones: HashMap::new(),
            root_position: position,
        };

        // Визначаємо параметри кісток
        skeleton.define_bones();

        // Створюємо фізичні тіла
        skeleton.create_bodies(physics, position);

        // Створюємо joints
        skeleton.create_joints(physics);

        skeleton
    }

    /// Визначає параметри всіх кісток (оптимізовано: 11 кісток)
    fn define_bones(&mut self) {
        // Торс - 3 кістки
        self.bones.insert(BoneId::Pelvis, Bone {
            id: BoneId::Pelvis,
            length: 0.2,
            radius: 0.12,
            mass: 10.0,
            local_offset: Vec3::ZERO,
            angle_limits: AngleLimits::free(), // Root - без обмежень
        });

        // Spine - об'єднує старий Spine + Chest
        self.bones.insert(BoneId::Spine, Bone {
            id: BoneId::Spine,
            length: 0.45,  // Довший - об'єднує spine+chest
            radius: 0.11,
            mass: 8.0,
            local_offset: Vec3::new(0.0, 0.2, 0.0),
            angle_limits: AngleLimits::spine(),
        });

        // Head - об'єднує Neck + Head
        self.bones.insert(BoneId::Head, Bone {
            id: BoneId::Head,
            length: 0.25,  // Включає шию
            radius: 0.09,
            mass: 4.5,
            local_offset: Vec3::new(0.0, 0.45, 0.0),  // Від верху spine
            angle_limits: AngleLimits::neck(),
        });

        // Ліва рука - 2 кістки (arms connect directly to spine)
        self.bones.insert(BoneId::LeftUpperArm, Bone {
            id: BoneId::LeftUpperArm,
            length: 0.28,
            radius: 0.04,
            mass: 2.0,
            local_offset: Vec3::new(-0.15, 0.35, 0.0),  // Від spine
            angle_limits: AngleLimits::shoulder(),
        });

        self.bones.insert(BoneId::LeftLowerArm, Bone {
            id: BoneId::LeftLowerArm,
            length: 0.25,
            radius: 0.035,
            mass: 1.0,
            local_offset: Vec3::new(-0.28, 0.0, 0.0),
            angle_limits: AngleLimits::elbow(),
        });

        // Права рука - 2 кістки
        self.bones.insert(BoneId::RightUpperArm, Bone {
            id: BoneId::RightUpperArm,
            length: 0.28,
            radius: 0.04,
            mass: 2.0,
            local_offset: Vec3::new(0.15, 0.35, 0.0),  // Від spine
            angle_limits: AngleLimits::shoulder(),
        });

        self.bones.insert(BoneId::RightLowerArm, Bone {
            id: BoneId::RightLowerArm,
            length: 0.25,
            radius: 0.035,
            mass: 1.0,
            local_offset: Vec3::new(0.28, 0.0, 0.0),
            angle_limits: AngleLimits::elbow(),
        });

        // Ліва нога - 2 кістки
        self.bones.insert(BoneId::LeftUpperLeg, Bone {
            id: BoneId::LeftUpperLeg,
            length: 0.42,
            radius: 0.06,
            mass: 5.0,
            local_offset: Vec3::new(-0.1, -0.1, 0.0),
            angle_limits: AngleLimits::hip(),
        });

        self.bones.insert(BoneId::LeftLowerLeg, Bone {
            id: BoneId::LeftLowerLeg,
            length: 0.40,
            radius: 0.045,
            mass: 3.0,
            local_offset: Vec3::new(0.0, -0.42, 0.0),
            angle_limits: AngleLimits::knee(),
        });

        // Права нога - 2 кістки
        self.bones.insert(BoneId::RightUpperLeg, Bone {
            id: BoneId::RightUpperLeg,
            length: 0.42,
            radius: 0.06,
            mass: 5.0,
            local_offset: Vec3::new(0.1, -0.1, 0.0),
            angle_limits: AngleLimits::hip(),
        });

        self.bones.insert(BoneId::RightLowerLeg, Bone {
            id: BoneId::RightLowerLeg,
            length: 0.40,
            radius: 0.045,
            mass: 3.0,
            local_offset: Vec3::new(0.0, -0.42, 0.0),
            angle_limits: AngleLimits::knee(),
        });
    }

    /// Створює фізичні тіла для кісток
    fn create_bodies(&mut self, physics: &mut PhysicsWorld, root_pos: Vec3) {
        // Обчислюємо world positions для кожної кістки
        let mut world_positions: HashMap<BoneId, Vec3> = HashMap::new();

        for bone_id in BoneId::all_bones() {
            let bone = self.bones.get(&bone_id).unwrap();

            let world_pos = if let Some(parent_id) = bone_id.parent() {
                let parent_pos = world_positions.get(&parent_id).unwrap();
                *parent_pos + bone.local_offset
            } else {
                root_pos
            };

            world_positions.insert(bone_id, world_pos);

            // Всі тіла динамічні, але з різним damping
            // Pelvis має дуже високий damping для стабільності
            let is_pelvis = bone_id == BoneId::Pelvis;
            let is_spine = bone_id == BoneId::Spine;
            let is_lower_leg = matches!(bone_id, BoneId::LeftLowerLeg | BoneId::RightLowerLeg);

            // Damping залежить від частини тіла
            let (angular_damp, linear_damp) = if is_pelvis {
                // Pelvis - дуже високий damping для стабільності
                (20.0, 10.0)
            } else if is_spine {
                // Хребет - високий damping
                (15.0, 5.0)
            } else if is_lower_leg {
                // Нижні частини ніг - середній (контакт з землею)
                (8.0, 3.0)
            } else {
                // Кінцівки - нижчий для природного руху
                (5.0, 1.0)
            };

            let body = RigidBodyBuilder::dynamic()
                .translation(vector![world_pos.x, world_pos.y, world_pos.z])
                .angular_damping(angular_damp)
                .linear_damping(linear_damp)
                .ccd_enabled(true)
                .build();

            let handle = physics.add_rigid_body(body);
            self.bodies.insert(bone_id, handle);

            // Створюємо collider (capsule) з collision filtering
            // КРИТИЧНО: Вимикаємо самозіткнення - це найдорожча операція!
            let collision_groups = InteractionGroups::new(
                Group::GROUP_1,  // This ragdoll's group
                Group::ALL & !Group::GROUP_1  // Collide with everything EXCEPT self
            );

            let collider = ColliderBuilder::capsule_y(bone.length / 2.0, bone.radius)
                .density(bone.mass / (std::f32::consts::PI * bone.radius * bone.radius * bone.length))
                .friction(0.8)
                .restitution(0.1)
                .collision_groups(collision_groups)  // Вимикаємо self-collision
                .build();

            physics.add_collider(collider, handle);
        }
    }

    /// Створює joints між кістками (оптимізовано: спеціалізовані типи)
    fn create_joints(&mut self, physics: &mut PhysicsWorld) {
        for bone_id in BoneId::all_bones() {
            if let Some(parent_id) = bone_id.parent() {
                let bone = self.bones.get(&bone_id).unwrap();
                let parent_handle = *self.bodies.get(&parent_id).unwrap();
                let child_handle = *self.bodies.get(&bone_id).unwrap();

                let anchor1 = point![bone.local_offset.x, bone.local_offset.y, bone.local_offset.z];
                let anchor2 = point![0.0, 0.0, 0.0];

                // Використовуємо різні типи joints для оптимізації
                match bone_id {
                    // HINGE JOINTS (1 DOF) - knees and elbows - 3x faster!
                    BoneId::LeftLowerLeg | BoneId::RightLowerLeg => {
                        // Knee - hinge around X axis
                        let mut joint = RevoluteJointBuilder::new(UnitVector::new_normalize(vector![1.0, 0.0, 0.0]))
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .limits([0.0, 2.5])  // 0° to ~140°
                            .motor_position(0.0, 150.0, 30.0)
                            .motor_max_force(1500.0)
                            .build();

                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    },

                    BoneId::LeftLowerArm | BoneId::RightLowerArm => {
                        // Elbow - hinge around X axis
                        let mut joint = RevoluteJointBuilder::new(UnitVector::new_normalize(vector![1.0, 0.0, 0.0]))
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .limits([0.0, 2.4])  // 0° to ~135°
                            .motor_position(0.0, 120.0, 25.0)
                            .motor_max_force(1200.0)
                            .build();

                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    },

                    // BALL JOINTS (3 DOF) - hips, shoulders, spine, head - 1.5x faster than Generic
                    BoneId::LeftUpperLeg | BoneId::RightUpperLeg => {
                        // Hip - ball socket
                        let joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();

                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    },

                    BoneId::LeftUpperArm | BoneId::RightUpperArm => {
                        // Shoulder - ball socket
                        let joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();

                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    },

                    BoneId::Spine => {
                        // Spine-Pelvis - ball socket with limited range
                        let joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();

                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    },

                    BoneId::Head => {
                        // Head-Spine - ball socket
                        let joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();

                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    },

                    _ => {
                        // Fallback - shouldn't happen with 11-bone skeleton
                        let mut joint = GenericJointBuilder::new(JointAxesMask::LOCKED_SPHERICAL_AXES)
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();
                        let joint_handle = physics.add_joint(parent_handle, child_handle, joint);
                        self.joints.insert(bone_id, joint_handle);
                    }
                }
            }
        }
    }

    /// Отримує позицію кістки
    pub fn get_bone_position(&self, physics: &PhysicsWorld, bone_id: BoneId) -> Option<Vec3> {
        self.bodies.get(&bone_id)
            .and_then(|handle| physics.get_body_position(*handle))
    }

    /// Отримує ротацію кістки
    pub fn get_bone_rotation(&self, physics: &PhysicsWorld, bone_id: BoneId) -> Option<Quat> {
        self.bodies.get(&bone_id)
            .and_then(|handle| physics.get_body_rotation(*handle))
    }

    /// Встановлює цільову ротацію для joint (motor)
    pub fn set_joint_target(
        &self,
        physics: &mut PhysicsWorld,
        bone_id: BoneId,
        target_angles: Vec3,  // (swing_x, twist, swing_z)
        stiffness: f32,
        damping: f32,
    ) {
        if let Some(joint_handle) = self.joints.get(&bone_id) {
            if let Some(joint) = physics.impulse_joint_set.get_mut(*joint_handle) {
                joint.data.set_motor_position(JointAxis::AngX, target_angles.x, stiffness, damping);
                joint.data.set_motor_position(JointAxis::AngY, target_angles.y, stiffness, damping);
                joint.data.set_motor_position(JointAxis::AngZ, target_angles.z, stiffness, damping);
            }
        }
    }

    /// Оновлює позицію та ротацію кінематичного pelvis
    /// Це основний спосіб керування персонажем
    pub fn set_pelvis_transform(
        &self,
        physics: &mut PhysicsWorld,
        position: Vec3,
        rotation: Quat,
    ) {
        if let Some(handle) = self.bodies.get(&BoneId::Pelvis) {
            if let Some(body) = physics.rigid_body_set.get_mut(*handle) {
                body.set_next_kinematic_position(Isometry::from_parts(
                    Translation::new(position.x, position.y, position.z),
                    nalgebra::UnitQuaternion::from_quaternion(
                        nalgebra::Quaternion::new(rotation.w, rotation.x, rotation.y, rotation.z)
                    ),
                ));
            }
        }
    }

    /// Отримує angular velocity кістки
    pub fn get_bone_angular_velocity(&self, physics: &PhysicsWorld, bone_id: BoneId) -> Option<Vec3> {
        self.bodies.get(&bone_id)
            .and_then(|handle| physics.rigid_body_set.get(*handle))
            .map(|body| {
                let av = body.angvel();
                Vec3::new(av.x, av.y, av.z)
            })
    }
}
