/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/physics/muscle.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Система м'язів - PD-контролери для керування суглобами.
   М'яз застосовує torque до кістки щоб досягти цільової пози.

🔬 ПРИНЦИП PD-КОНТРОЛЕРА:
   torque = Kp * (target_angle - current_angle) + Kd * (0 - angular_velocity)

   Kp (Proportional) - жорсткість м'яза (як сильно тягне до цілі)
   Kd (Derivative) - демпфування (запобігає осциляціям)

═══════════════════════════════════════════════════════════════════════════════
*/

use glam::{Vec3, Quat};
use std::collections::HashMap;

use super::skeleton::{Skeleton, BoneId};

/// Smooth step function для плавної інтерполяції
/// Ease-in-ease-out: повільний старт, швидка середина, повільний кінець
/// Формула: t² × (3 - 2t)
#[inline]
pub fn smooth_step(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

use super::PhysicsWorld;

/// PD-контролер для одного суглоба
#[derive(Debug, Clone)]
pub struct Muscle {
    /// Кістка яку контролює м'яз
    pub bone_id: BoneId,

    /// Proportional gain (жорсткість)
    pub kp: f32,

    /// Derivative gain (демпфування)
    pub kd: f32,

    /// Максимальний torque який може видати м'яз
    pub max_torque: f32,

    /// Цільова ротація (local space відносно батька)
    pub target_rotation: Quat,

    /// Сила м'яза (0.0 = розслаблений, 1.0 = максимальна напруга)
    pub strength: f32,
}

impl Muscle {
    /// Створює новий м'яз
    pub fn new(bone_id: BoneId, kp: f32, kd: f32, max_torque: f32) -> Self {
        Self {
            bone_id,
            kp,
            kd,
            max_torque,
            target_rotation: Quat::IDENTITY,
            strength: 1.0,
        }
    }

    /// Встановлює цільову ротацію
    pub fn set_target(&mut self, rotation: Quat) {
        self.target_rotation = rotation;
    }

    /// Встановлює цільову ротацію з Euler кутів (pitch, yaw, roll)
    pub fn set_target_euler(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.target_rotation = Quat::from_euler(glam::EulerRot::XYZ, pitch, yaw, roll);
    }

    /// Обчислює torque для досягнення цільової пози
    pub fn calculate_torque(
        &self,
        current_rotation: Quat,
        angular_velocity: Vec3,
    ) -> Vec3 {
        if self.strength < 0.01 {
            return Vec3::ZERO;
        }

        // Обчислюємо різницю ротацій
        // error = target * inverse(current)
        let error_quat = self.target_rotation * current_rotation.inverse();

        // Конвертуємо quaternion error в axis-angle
        let (axis, angle) = error_quat.to_axis_angle();

        // Забезпечуємо найкоротший шлях
        let angle = if angle > std::f32::consts::PI {
            angle - std::f32::consts::TAU
        } else if angle < -std::f32::consts::PI {
            angle + std::f32::consts::TAU
        } else {
            angle
        };

        // PD control
        // P term: пропорційний до помилки
        let p_term = axis * angle * self.kp;

        // D term: демпфування на основі angular velocity
        let d_term = -angular_velocity * self.kd;

        // Сумарний torque
        let mut torque = (p_term + d_term) * self.strength;

        // Обмежуємо максимальний torque
        let torque_magnitude = torque.length();
        if torque_magnitude > self.max_torque {
            torque = torque.normalize() * self.max_torque;
        }

        torque
    }
}

/// Система м'язів для всього скелета
pub struct MuscleSystem {
    /// М'язи для кожної кістки
    pub muscles: HashMap<BoneId, Muscle>,

    /// Глобальний множник сили (для ragdoll ефекту)
    pub global_strength: f32,
}

impl MuscleSystem {
    /// Створює систему м'язів для гуманоїдного скелета (оптимізовано: 11 кісток)
    pub fn create_humanoid() -> Self {
        let mut muscles = HashMap::new();

        // Торс - сильні м'язи для підтримки вертикального положення
        muscles.insert(BoneId::Spine, Muscle::new(BoneId::Spine, 800.0, 80.0, 500.0));

        // Голова (merged neck + head)
        muscles.insert(BoneId::Head, Muscle::new(BoneId::Head, 250.0, 25.0, 120.0));

        // Руки - upper and lower arm only
        muscles.insert(BoneId::LeftUpperArm, Muscle::new(BoneId::LeftUpperArm, 400.0, 40.0, 200.0));
        muscles.insert(BoneId::LeftLowerArm, Muscle::new(BoneId::LeftLowerArm, 300.0, 30.0, 150.0));

        muscles.insert(BoneId::RightUpperArm, Muscle::new(BoneId::RightUpperArm, 400.0, 40.0, 200.0));
        muscles.insert(BoneId::RightLowerArm, Muscle::new(BoneId::RightLowerArm, 300.0, 30.0, 150.0));

        // Ноги - upper and lower leg only
        muscles.insert(BoneId::LeftUpperLeg, Muscle::new(BoneId::LeftUpperLeg, 1000.0, 100.0, 800.0));
        muscles.insert(BoneId::LeftLowerLeg, Muscle::new(BoneId::LeftLowerLeg, 800.0, 80.0, 600.0));

        muscles.insert(BoneId::RightUpperLeg, Muscle::new(BoneId::RightUpperLeg, 1000.0, 100.0, 800.0));
        muscles.insert(BoneId::RightLowerLeg, Muscle::new(BoneId::RightLowerLeg, 800.0, 80.0, 600.0));

        Self {
            muscles,
            global_strength: 1.0,
        }
    }

    /// Оновлює м'язи і застосовує torque до фізичних тіл
    pub fn update(&self, physics: &mut PhysicsWorld, skeleton: &Skeleton) {
        for (bone_id, muscle) in &self.muscles {
            // Отримуємо поточну ротацію кістки
            if let Some(body_handle) = skeleton.bodies.get(bone_id) {
                if let Some(body) = physics.rigid_body_set.get(*body_handle) {
                    let current_rotation = super::rapier_to_quat(body.rotation());
                    let angular_velocity = super::rapier_to_vec3(body.angvel());

                    // Обчислюємо torque
                    let mut torque = muscle.calculate_torque(current_rotation, angular_velocity);

                    // Застосовуємо глобальний множник
                    torque *= self.global_strength;

                    // Застосовуємо torque
                    physics.apply_torque(*body_handle, torque);
                }
            }
        }
    }

    /// Встановлює цільову позу для всіх м'язів
    pub fn set_pose(&mut self, pose: &TargetPose) {
        for (bone_id, rotation) in &pose.bone_rotations {
            if let Some(muscle) = self.muscles.get_mut(bone_id) {
                muscle.set_target(*rotation);
            }
        }
    }

    /// Встановлює силу конкретного м'яза
    pub fn set_muscle_strength(&mut self, bone_id: BoneId, strength: f32) {
        if let Some(muscle) = self.muscles.get_mut(&bone_id) {
            muscle.strength = strength.clamp(0.0, 1.0);
        }
    }

    /// Робить всі м'язи розслабленими (ragdoll mode)
    pub fn go_ragdoll(&mut self) {
        self.global_strength = 0.0;
    }

    /// Відновлює контроль над м'язами
    pub fn recover(&mut self) {
        self.global_strength = 1.0;
    }
}

/// Цільова поза - набір ротацій для всіх кісток
#[derive(Debug, Clone)]
pub struct TargetPose {
    pub bone_rotations: HashMap<BoneId, Quat>,
}

impl TargetPose {
    /// Створює нейтральну T-позу
    pub fn t_pose() -> Self {
        let mut rotations = HashMap::new();

        // Все в neutral position
        for bone_id in BoneId::all_bones() {
            rotations.insert(bone_id, Quat::IDENTITY);
        }

        // Руки горизонтально
        rotations.insert(BoneId::LeftUpperArm, Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2));
        rotations.insert(BoneId::RightUpperArm, Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

        Self { bone_rotations: rotations }
    }

    /// Створює позу стояння
    pub fn standing() -> Self {
        let mut rotations = HashMap::new();

        for bone_id in BoneId::all_bones() {
            rotations.insert(bone_id, Quat::IDENTITY);
        }

        // Руки трохи опущені
        rotations.insert(BoneId::LeftUpperArm, Quat::from_rotation_z(-0.3));
        rotations.insert(BoneId::RightUpperArm, Quat::from_rotation_z(0.3));

        // Лікті трохи зігнуті
        rotations.insert(BoneId::LeftLowerArm, Quat::from_rotation_x(0.2));
        rotations.insert(BoneId::RightLowerArm, Quat::from_rotation_x(0.2));

        Self { bone_rotations: rotations }
    }

    /// Інтерполює між двома позами
    pub fn lerp(a: &TargetPose, b: &TargetPose, t: f32) -> Self {
        let mut rotations = HashMap::new();

        for bone_id in BoneId::all_bones() {
            let rot_a = a.bone_rotations.get(&bone_id).copied().unwrap_or(Quat::IDENTITY);
            let rot_b = b.bone_rotations.get(&bone_id).copied().unwrap_or(Quat::IDENTITY);

            rotations.insert(bone_id, rot_a.slerp(rot_b, t));
        }

        Self { bone_rotations: rotations }
    }
}

/// Цикл ходьби - генерує пози для анімації ходьби
#[derive(Debug, Clone)]
pub struct WalkCycle {
    /// Фаза циклу (0.0 - 1.0)
    pub phase: f32,

    /// Швидкість ходьби
    pub speed: f32,

    /// Довжина кроку (радіани повороту стегна)
    pub stride_length: f32,

    /// Висота підйому ноги
    pub step_height: f32,

    /// Бокове розгойдування стегон
    pub hip_sway: f32,

    /// Нахил торсу вперед при ходьбі/бігу
    pub spine_lean_forward: f32,

    /// Амплітуда розмаху рук
    pub arm_swing_amount: f32,
}

impl WalkCycle {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            speed: 1.0,
            stride_length: 0.5,       // радіани (~30°)
            step_height: 0.15,        // висота підйому ноги
            hip_sway: 0.05,           // бокове розгойдування
            spine_lean_forward: 0.1,  // нахил вперед при русі
            arm_swing_amount: 0.3,    // розмах рук
        }
    }

    /// Оновлює фазу циклу
    pub fn update(&mut self, delta: f32, is_walking: bool) {
        if is_walking {
            self.phase += delta * self.speed * 2.0;  // ~2 кроки за секунду
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    /// Генерує цільову позу для поточної фази
    pub fn get_pose(&self) -> TargetPose {
        let mut rotations = HashMap::new();

        // Base pose
        for bone_id in BoneId::all_bones() {
            rotations.insert(bone_id, Quat::IDENTITY);
        }

        // Застосовуємо smooth_step для плавної анімації
        // phase 0.0-1.0 → smoothed phase для ease-in-ease-out
        let smoothed_phase = smooth_step(self.phase);
        let phase_rad = smoothed_phase * std::f32::consts::TAU;

        // Ноги - використовуємо stride_length параметр
        let leg_swing = phase_rad.sin() * self.stride_length;

        // Ліва нога
        rotations.insert(BoneId::LeftUpperLeg, Quat::from_rotation_x(-leg_swing));
        // Коліно згинається коли нога позаду + step_height впливає на підйом
        let left_knee_bend = ((-leg_swing).max(0.0) * (1.5 + self.step_height)).min(1.2);
        rotations.insert(BoneId::LeftLowerLeg, Quat::from_rotation_x(left_knee_bend));

        // Права нога (протилежна фаза)
        rotations.insert(BoneId::RightUpperLeg, Quat::from_rotation_x(leg_swing));
        let right_knee_bend = ((leg_swing).max(0.0) * (1.5 + self.step_height)).min(1.2);
        rotations.insert(BoneId::RightLowerLeg, Quat::from_rotation_x(right_knee_bend));

        // Руки - протилежно ногам, використовуємо arm_swing_amount
        let arm_swing = phase_rad.sin() * self.arm_swing_amount;
        rotations.insert(BoneId::LeftUpperArm,
            Quat::from_rotation_z(-0.2) * Quat::from_rotation_x(arm_swing));
        rotations.insert(BoneId::RightUpperArm,
            Quat::from_rotation_z(0.2) * Quat::from_rotation_x(-arm_swing));

        // Лікті завжди трохи зігнуті
        rotations.insert(BoneId::LeftLowerArm, Quat::from_rotation_x(0.3));
        rotations.insert(BoneId::RightLowerArm, Quat::from_rotation_x(0.3));

        // Торс - обертання + нахил вперед пропорційно швидкості
        let torso_twist = phase_rad.sin() * 0.1;
        let forward_lean = -self.spine_lean_forward * (self.speed / 3.0).min(1.0);
        rotations.insert(BoneId::Spine,
            Quat::from_rotation_x(forward_lean) * Quat::from_rotation_y(torso_twist));

        TargetPose { bone_rotations: rotations }
    }
}

impl Default for WalkCycle {
    fn default() -> Self {
        Self::new()
    }
}
