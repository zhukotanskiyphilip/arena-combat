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
use glam::{Vec3, Quat};
use std::collections::HashMap;

use super::PhysicsWorld;
use crate::debug_log::log_debug;

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

    /// Impulse joint handles (краща стабільність для active ragdoll)
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
    ///
    /// ПРОПОРЦІЇ З РЕФЕРЕНСНОГО ЗОБРАЖЕННЯ (математично виміряні)
    /// Загальна висота = 1.8м, всі пропорції як частки від висоти
    ///
    /// ВЕРТИКАЛЬНІ ПОЗИЦІЇ (від землі):
    /// - 0.00 (0.00м) - ground
    /// - 0.03 (0.05м) - ankles
    /// - 0.25 (0.45м) - knees (1/4 висоти!)
    /// - 0.50 (0.90м) - crotch (ТОЧНО середина!)
    /// - 0.62 (1.12м) - elbows
    /// - 0.84 (1.51м) - shoulders
    /// - 0.88 (1.58м) - chin
    /// - 1.00 (1.80м) - crown
    fn define_bones(&mut self) {
        // === БАЗОВІ КОНСТАНТИ ===
        const TOTAL_HEIGHT: f32 = 1.80;

        // === ВЕРТИКАЛЬНІ ПОЗИЦІЇ (пропорції × 1.8м) ===
        // const GROUND: f32 = 0.0;
        // const ANKLE: f32 = 0.05;      // 0.03 × 1.8
        const KNEE: f32 = 0.45;          // 0.25 × 1.8
        const CROTCH: f32 = 0.90;        // 0.50 × 1.8 - СЕРЕДИНА!
        // const ELBOW: f32 = 1.12;      // 0.62 × 1.8
        const SHOULDER: f32 = 1.51;      // 0.84 × 1.8
        // const CHIN: f32 = 1.58;       // 0.88 × 1.8
        // const CROWN: f32 = 1.80;      // 1.00 × 1.8

        // === ШИРИНИ (пропорції × 1.8м) ===
        const SHOULDER_HALF_WIDTH: f32 = 0.43;  // 0.24 × 1.8 - від центру до краю плеча
        const HIP_HALF_WIDTH: f32 = 0.14;       // 0.08 × 1.8 - від центру до hip joint
        const CHEST_RADIUS: f32 = 0.16;         // 0.09 × 1.8 - радіус грудей
        const PELVIS_RADIUS: f32 = 0.14;        // 0.08 × 1.8 - радіус тазу

        // === ДІАМЕТРИ КІНЦІВОК (пропорції × 1.8м) ===
        const THIGH_RADIUS: f32 = 0.08;         // 0.045 × 1.8 - радіус стегна
        const CALF_RADIUS: f32 = 0.045;         // 0.025 × 1.8 - радіус гомілки
        const BICEP_RADIUS: f32 = 0.05;         // 0.028 × 1.8 - радіус біцепса
        const FOREARM_RADIUS: f32 = 0.036;      // 0.02 × 1.8 - радіус передпліччя
        const HEAD_RADIUS: f32 = 0.09;          // 0.05 × 1.8 - радіус голови

        // === ДОВЖИНИ СЕГМЕНТІВ (пропорції × 1.8м) ===
        const HEAD_LENGTH: f32 = 0.22;          // 0.12 × 1.8
        const NECK_LENGTH: f32 = 0.07;          // 0.04 × 1.8
        const TORSO_LENGTH: f32 = 0.61;         // 0.34 × 1.8 (shoulders to crotch)
        const THIGH_LENGTH: f32 = 0.45;         // 0.25 × 1.8 (crotch to knee)
        const CALF_LENGTH: f32 = 0.40;          // 0.22 × 1.8 (knee to ankle)
        const UPPER_ARM_LENGTH: f32 = 0.32;     // 0.18 × 1.8
        const FOREARM_LENGTH: f32 = 0.29;       // 0.16 × 1.8

        // Shoulder joint виступає за торс
        const SHOULDER_OFFSET: f32 = SHOULDER_HALF_WIDTH - CHEST_RADIUS + 0.02;  // ~0.29м від центру

        // === ТОРС (3 кістки) ===
        // Pelvis + Spine разом = TORSO_LENGTH (0.61м)
        // Розділяємо: Pelvis ~0.15м, Spine ~0.46м

        // Pelvis: таз - нижня частина торсу
        self.bones.insert(BoneId::Pelvis, Bone {
            id: BoneId::Pelvis,
            length: 0.15,
            radius: PELVIS_RADIUS,  // 0.14м
            mass: 12.0,
            local_offset: Vec3::ZERO,
            angle_limits: AngleLimits::free(),
        });

        // Spine: від тазу до плечей (основна частина торсу)
        self.bones.insert(BoneId::Spine, Bone {
            id: BoneId::Spine,
            length: TORSO_LENGTH - 0.15,  // 0.46м
            radius: CHEST_RADIUS,         // 0.16м - широкі груди
            mass: 10.0,
            local_offset: Vec3::new(0.0, 0.075, 0.0),  // Pelvis length/2
            angle_limits: AngleLimits::spine(),
        });

        // Head: голова + шия
        self.bones.insert(BoneId::Head, Bone {
            id: BoneId::Head,
            length: HEAD_LENGTH + NECK_LENGTH,  // 0.29м
            radius: HEAD_RADIUS,                // 0.09м
            mass: 5.0,
            local_offset: Vec3::new(0.0, 0.23, 0.0),  // Spine length/2
            angle_limits: AngleLimits::neck(),
        });

        // === РУКИ (4 кістки) ===
        // З референсу: upper arm = 0.32м, forearm = 0.29м

        // Ліва рука: плече (upper arm / bicep)
        self.bones.insert(BoneId::LeftUpperArm, Bone {
            id: BoneId::LeftUpperArm,
            length: UPPER_ARM_LENGTH,   // 0.32м
            radius: BICEP_RADIUS,       // 0.05м
            mass: 2.5,
            // Плече кріпиться збоку від spine, трохи нижче верху
            local_offset: Vec3::new(-SHOULDER_OFFSET, 0.15, 0.0),
            angle_limits: AngleLimits::shoulder(),
        });

        // Ліва рука: передпліччя (forearm)
        self.bones.insert(BoneId::LeftLowerArm, Bone {
            id: BoneId::LeftLowerArm,
            length: FOREARM_LENGTH,     // 0.29м
            radius: FOREARM_RADIUS,     // 0.036м
            mass: 1.5,
            local_offset: Vec3::new(0.0, -UPPER_ARM_LENGTH, 0.0),
            angle_limits: AngleLimits::elbow(),
        });

        // Права рука: плече (upper arm / bicep)
        self.bones.insert(BoneId::RightUpperArm, Bone {
            id: BoneId::RightUpperArm,
            length: UPPER_ARM_LENGTH,
            radius: BICEP_RADIUS,
            mass: 2.5,
            local_offset: Vec3::new(SHOULDER_OFFSET, 0.15, 0.0),
            angle_limits: AngleLimits::shoulder(),
        });

        // Права рука: передпліччя (forearm)
        self.bones.insert(BoneId::RightLowerArm, Bone {
            id: BoneId::RightLowerArm,
            length: FOREARM_LENGTH,
            radius: FOREARM_RADIUS,
            mass: 1.5,
            local_offset: Vec3::new(0.0, -UPPER_ARM_LENGTH, 0.0),
            angle_limits: AngleLimits::elbow(),
        });

        // === НОГИ (4 кістки) ===
        // З референсу: thigh = 0.45м, calf = 0.40м
        // Total leg = 0.85м (crotch 0.90м - ankle 0.05м)

        // Ліва нога: стегно (thigh)
        self.bones.insert(BoneId::LeftUpperLeg, Bone {
            id: BoneId::LeftUpperLeg,
            length: THIGH_LENGTH,       // 0.45м
            radius: THIGH_RADIUS,       // 0.08м - масивне
            mass: 8.0,
            // Кріпиться до низу pelvis, збоку
            local_offset: Vec3::new(-HIP_HALF_WIDTH, -0.075, 0.0),
            angle_limits: AngleLimits::hip(),
        });

        // Ліва нога: гомілка (calf)
        self.bones.insert(BoneId::LeftLowerLeg, Bone {
            id: BoneId::LeftLowerLeg,
            length: CALF_LENGTH,        // 0.40м
            radius: CALF_RADIUS,        // 0.045м - тонша
            mass: 4.0,
            local_offset: Vec3::new(0.0, -THIGH_LENGTH, 0.0),
            angle_limits: AngleLimits::knee(),
        });

        // Права нога: стегно (thigh)
        self.bones.insert(BoneId::RightUpperLeg, Bone {
            id: BoneId::RightUpperLeg,
            length: THIGH_LENGTH,
            radius: THIGH_RADIUS,
            mass: 8.0,
            local_offset: Vec3::new(HIP_HALF_WIDTH, -0.075, 0.0),
            angle_limits: AngleLimits::hip(),
        });

        // Права нога: гомілка (calf)
        self.bones.insert(BoneId::RightLowerLeg, Bone {
            id: BoneId::RightLowerLeg,
            length: CALF_LENGTH,
            radius: CALF_RADIUS,
            mass: 4.0,
            local_offset: Vec3::new(0.0, -THIGH_LENGTH, 0.0),
            angle_limits: AngleLimits::knee(),
        });
    }

    /// Створює фізичні тіла для кісток
    fn create_bodies(&mut self, physics: &mut PhysicsWorld, root_pos: Vec3) {
        log_debug("=== SKELETON CREATION ===");
        log_debug(&format!("Root position: ({:.2}, {:.2}, {:.2})", root_pos.x, root_pos.y, root_pos.z));

        // Обчислюємо world positions для ЦЕНТРІВ кісток (не точок з'єднання!)
        // Це критично важливо - Rapier позиціонує тіла по центру
        let mut world_positions: HashMap<BoneId, Vec3> = HashMap::new();

        for bone_id in BoneId::all_bones() {
            let bone = self.bones.get(&bone_id).unwrap();

            let world_pos = if let Some(parent_id) = bone_id.parent() {
                let parent_pos = world_positions.get(&parent_id).unwrap();
                let parent_bone = self.bones.get(&parent_id).unwrap();

                // Точка з'єднання на батьківській кістці
                // Для рук: local_offset.x визначає відстань до плечового суглоба
                let joint_point = *parent_pos + bone.local_offset;

                // Зміщення від точки з'єднання до центру дочірньої кістки
                // Залежить від того, яким кінцем кістка кріпиться
                // A-POSE: руки відведені від тіла на ~25 градусів
                let half_len = bone.length / 2.0;

                // Кут відведення рук для A-pose (~25 градусів = 0.44 радіан)
                const ARM_ANGLE: f32 = 0.44;  // ~25 degrees from vertical
                let arm_x = half_len * ARM_ANGLE.sin();  // Horizontal component
                let arm_y = half_len * ARM_ANGLE.cos();  // Vertical component

                let center_offset = match bone_id {
                    // Ноги: верхній кінець (+Y) кріпиться до батька → центр нижче на half_len
                    BoneId::LeftUpperLeg | BoneId::RightUpperLeg |
                    BoneId::LeftLowerLeg | BoneId::RightLowerLeg => {
                        Vec3::new(0.0, -half_len, 0.0)
                    }
                    // Spine/Head: нижній кінець (-Y) кріпиться до батька → центр вище на half_len
                    BoneId::Spine | BoneId::Head => {
                        Vec3::new(0.0, half_len, 0.0)
                    }
                    // A-POSE: Руки відведені від тіла
                    // Upper arms: кріпиться до spine, відведена назовні
                    BoneId::LeftUpperArm => {
                        Vec3::new(-arm_x, -arm_y, 0.0)  // Left: negative X
                    }
                    BoneId::RightUpperArm => {
                        Vec3::new(arm_x, -arm_y, 0.0)   // Right: positive X
                    }
                    // Lower arms: кріпиться до upper arm, продовжує напрямок
                    BoneId::LeftLowerArm => {
                        Vec3::new(-arm_x, -arm_y, 0.0)
                    }
                    BoneId::RightLowerArm => {
                        Vec3::new(arm_x, -arm_y, 0.0)
                    }
                    _ => Vec3::ZERO,
                };

                joint_point + center_offset
            } else {
                root_pos
            };

            world_positions.insert(bone_id, world_pos);

            // Логування створеної позиції
            log_debug(&format!(
                "{:?}: center=({:.3}, {:.3}, {:.3}) length={:.2} radius={:.2}",
                bone_id, world_pos.x, world_pos.y, world_pos.z, bone.length, bone.radius
            ));

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

            // Обчислюємо початкову ротацію
            // A-POSE: руки повернуті на ~25° від вертикалі
            const ARM_ANGLE: f32 = 0.44;  // ~25 degrees
            let initial_rotation = match bone_id {
                // Ліва рука: поворот навколо Z (нахил назовні)
                BoneId::LeftUpperArm | BoneId::LeftLowerArm => {
                    nalgebra::UnitQuaternion::from_axis_angle(
                        &nalgebra::Vector3::z_axis(),
                        -ARM_ANGLE  // Negative = rotate outward for left arm
                    )
                }
                // Права рука: поворот навколо Z (нахил назовні)
                BoneId::RightUpperArm | BoneId::RightLowerArm => {
                    nalgebra::UnitQuaternion::from_axis_angle(
                        &nalgebra::Vector3::z_axis(),
                        ARM_ANGLE  // Positive = rotate outward for right arm
                    )
                }
                // Всі інші: без ротації
                _ => nalgebra::UnitQuaternion::identity()
            };

            let body = RigidBodyBuilder::dynamic()
                .translation(vector![world_pos.x, world_pos.y, world_pos.z])
                .rotation(initial_rotation.scaled_axis())
                .angular_damping(angular_damp)
                .linear_damping(linear_damp)
                .ccd_enabled(true)
                .build();

            let handle = physics.add_rigid_body(body);
            self.bodies.insert(bone_id, handle);

            // Створюємо collider з collision filtering
            // ВИМКНЕНО самозіткнення - запобігає стрибанню кінцівок
            let collision_groups = InteractionGroups::new(
                Group::GROUP_1,
                Group::ALL & !Group::GROUP_1  // Collide with everything EXCEPT self
            );

            // ВСІ кістки - КАПСУЛИ (capsule_y)
            // Це дає правильну форму як на референсі
            let collider = ColliderBuilder::capsule_y(bone.length / 2.0, bone.radius)
                .density(bone.mass / (std::f32::consts::PI * bone.radius * bone.radius * bone.length))
                .friction(0.8)
                .restitution(0.1)
                .collision_groups(collision_groups)
                .build();

            physics.add_collider(collider, handle);
        }
    }

    /// Створює joints між кістками (MULTIBODY - reduced coordinates, cannot violate constraints!)
    fn create_joints(&mut self, physics: &mut PhysicsWorld) {
        log_debug("=== MULTIBODY JOINTS CREATION ===");

        for bone_id in BoneId::all_bones() {
            if let Some(parent_id) = bone_id.parent() {
                let bone = self.bones.get(&bone_id).unwrap();
                let parent_handle = *self.bodies.get(&parent_id).unwrap();
                let child_handle = *self.bodies.get(&bone_id).unwrap();

                // anchor1: точка кріплення на батьківській кістці (в ЛОКАЛЬНИХ координатах батька)
                // local_offset - це зміщення від ЦЕНТРУ батька до точки з'єднання
                let parent_bone = self.bones.get(&parent_id).unwrap();
                let child_half_len = bone.length / 2.0;
                let parent_half_len = parent_bone.length / 2.0;

                // Anchor1 залежить від типу батьківської кістки:
                // Всі капсули вертикальні (Y-axis), центр в середині
                // anchor1 = точка кріплення на БАТЬКУ в локальних координатах батька
                //
                // ВАЖЛИВО: anchor повинен вказувати на КРАЙ кістки, а не на local_offset
                // local_offset використовується для ПОЗИЦІОНУВАННЯ тіла при створенні,
                // а anchor - для з'єднання в ЛОКАЛЬНИХ координатах (відносно центру тіла)
                let anchor1 = match (parent_id, bone_id) {
                    // Spine: кріпиться до верху pelvis
                    (BoneId::Pelvis, BoneId::Spine) => {
                        point![0.0, parent_half_len, 0.0]  // Верх pelvis
                    }
                    // Head: кріпиться до верху spine
                    (BoneId::Spine, BoneId::Head) => {
                        point![0.0, parent_half_len, 0.0]  // Верх spine
                    }
                    // Upper arms: кріпляться збоку spine, трохи нижче верху
                    (BoneId::Spine, BoneId::LeftUpperArm) => {
                        point![-0.29, 0.15, 0.0]  // Ліве плече
                    }
                    (BoneId::Spine, BoneId::RightUpperArm) => {
                        point![0.29, 0.15, 0.0]   // Праве плече
                    }
                    // Lower arms: кріпляться до НИЗУ upper arm
                    (BoneId::LeftUpperArm, _) | (BoneId::RightUpperArm, _) => {
                        point![0.0, -parent_half_len, 0.0]  // Низ upper arm
                    }
                    // Upper legs: кріпляться до НИЗУ pelvis, збоку
                    (BoneId::Pelvis, BoneId::LeftUpperLeg) => {
                        point![-0.14, -parent_half_len, 0.0]  // Ліве стегно
                    }
                    (BoneId::Pelvis, BoneId::RightUpperLeg) => {
                        point![0.14, -parent_half_len, 0.0]   // Праве стегно
                    }
                    // Lower legs: кріпляться до НИЗУ upper leg
                    (BoneId::LeftUpperLeg, _) | (BoneId::RightUpperLeg, _) => {
                        point![0.0, -parent_half_len, 0.0]  // Низ upper leg (коліно)
                    }
                    // Fallback
                    _ => point![bone.local_offset.x, bone.local_offset.y, bone.local_offset.z],
                };

                // anchor2: точка кріплення на дочірній кістці (в локальних координатах дитини)
                // Всі капсули вертикальні (Y-axis), верхній кінець = +Y, нижній = -Y
                let anchor2 = match bone_id {
                    // Ноги: верхній кінець (+Y) кріпиться до pelvis
                    BoneId::LeftUpperLeg | BoneId::RightUpperLeg |
                    BoneId::LeftLowerLeg | BoneId::RightLowerLeg => {
                        point![0.0, child_half_len, 0.0]
                    }
                    // Spine/Head: нижній кінець (-Y) кріпиться до батька
                    BoneId::Spine | BoneId::Head => {
                        point![0.0, -child_half_len, 0.0]
                    }
                    // Руки (всі вертикальні): верхній кінець (+Y) кріпиться до батька
                    BoneId::LeftUpperArm | BoneId::RightUpperArm |
                    BoneId::LeftLowerArm | BoneId::RightLowerArm => {
                        point![0.0, child_half_len, 0.0]
                    }
                    _ => point![0.0, 0.0, 0.0],
                };

                log_debug(&format!(
                    "{:?}->{:?}: anchor1=({:.3}, {:.3}, {:.3}) anchor2=({:.3}, {:.3}, {:.3})",
                    parent_id, bone_id,
                    anchor1.x, anchor1.y, anchor1.z,
                    anchor2.x, anchor2.y, anchor2.z
                ));

                // Використовуємо IMPULSE joints - краща стабільність для active ragdoll
                match bone_id {
                    // HINGE JOINTS (1 DOF) - knees and elbows
                    BoneId::LeftLowerLeg | BoneId::RightLowerLeg => {
                        let joint = RevoluteJointBuilder::new(UnitVector::new_normalize(vector![1.0, 0.0, 0.0]))
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .limits([0.0, 2.5])
                            .motor_position(0.0, 150.0, 30.0)
                            .motor_max_force(1500.0)
                            .build();

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
                        self.joints.insert(bone_id, joint_handle);
                        log_debug(&format!("Created ImpulseRevoluteJoint (knee) for {:?}", bone_id));
                    },

                    BoneId::LeftLowerArm | BoneId::RightLowerArm => {
                        let joint = RevoluteJointBuilder::new(UnitVector::new_normalize(vector![1.0, 0.0, 0.0]))
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .limits([0.0, 2.4])
                            .motor_position(0.0, 120.0, 25.0)
                            .motor_max_force(1200.0)
                            .build();

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
                        self.joints.insert(bone_id, joint_handle);
                        log_debug(&format!("Created ImpulseRevoluteJoint (elbow) for {:?}", bone_id));
                    },

                    // SPHERICAL JOINTS (3 DOF) - shoulders, hips, spine, head
                    // З motor для жорсткості суглобів
                    BoneId::LeftUpperLeg | BoneId::RightUpperLeg => {
                        // Hip joints - потужні для підтримки тіла
                        let mut joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();
                        // Додаємо motor на всіх осях для жорсткості
                        joint.set_motor_position(JointAxis::AngX, 0.0, 200.0, 40.0);
                        joint.set_motor_position(JointAxis::AngY, 0.0, 200.0, 40.0);
                        joint.set_motor_position(JointAxis::AngZ, 0.0, 200.0, 40.0);
                        joint.set_motor_max_force(JointAxis::AngX, 2000.0);
                        joint.set_motor_max_force(JointAxis::AngY, 2000.0);
                        joint.set_motor_max_force(JointAxis::AngZ, 2000.0);

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
                        self.joints.insert(bone_id, joint_handle);
                        log_debug(&format!("Created ImpulseSphericalJoint (hip) for {:?}", bone_id));
                    },

                    BoneId::LeftUpperArm | BoneId::RightUpperArm => {
                        // Shoulder joints - середня жорсткість
                        let mut joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();
                        joint.set_motor_position(JointAxis::AngX, 0.0, 100.0, 20.0);
                        joint.set_motor_position(JointAxis::AngY, 0.0, 100.0, 20.0);
                        joint.set_motor_position(JointAxis::AngZ, 0.0, 100.0, 20.0);
                        joint.set_motor_max_force(JointAxis::AngX, 1000.0);
                        joint.set_motor_max_force(JointAxis::AngY, 1000.0);
                        joint.set_motor_max_force(JointAxis::AngZ, 1000.0);

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
                        self.joints.insert(bone_id, joint_handle);
                        log_debug(&format!("Created ImpulseSphericalJoint (shoulder) for {:?}", bone_id));
                    },

                    BoneId::Spine => {
                        // Spine - дуже жорсткий для стабільності
                        let mut joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();
                        joint.set_motor_position(JointAxis::AngX, 0.0, 300.0, 60.0);
                        joint.set_motor_position(JointAxis::AngY, 0.0, 300.0, 60.0);
                        joint.set_motor_position(JointAxis::AngZ, 0.0, 300.0, 60.0);
                        joint.set_motor_max_force(JointAxis::AngX, 3000.0);
                        joint.set_motor_max_force(JointAxis::AngY, 3000.0);
                        joint.set_motor_max_force(JointAxis::AngZ, 3000.0);

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
                        self.joints.insert(bone_id, joint_handle);
                        log_debug(&format!("Created ImpulseSphericalJoint (spine) for {:?}", bone_id));
                    },

                    BoneId::Head => {
                        // Head/neck - м'якший для природного руху
                        let mut joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();
                        joint.set_motor_position(JointAxis::AngX, 0.0, 80.0, 15.0);
                        joint.set_motor_position(JointAxis::AngY, 0.0, 80.0, 15.0);
                        joint.set_motor_position(JointAxis::AngZ, 0.0, 80.0, 15.0);
                        joint.set_motor_max_force(JointAxis::AngX, 800.0);
                        joint.set_motor_max_force(JointAxis::AngY, 800.0);
                        joint.set_motor_max_force(JointAxis::AngZ, 800.0);

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
                        self.joints.insert(bone_id, joint_handle);
                        log_debug(&format!("Created ImpulseSphericalJoint (head) for {:?}", bone_id));
                    },

                    _ => {
                        let joint = SphericalJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2)
                            .build();

                        let joint_handle = physics.impulse_joint_set.insert(
                            parent_handle,
                            child_handle,
                            joint,
                            true
                        );
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
