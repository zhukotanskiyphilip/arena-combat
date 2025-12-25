/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/physics/ragdoll.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Active Ragdoll - гібридна система як в GTA 4 / RDR 2 / Hellish Quart.

   ПІДХІД:
   - Всі кістки динамічні (реагують на фізику)
   - Pelvis контролюється через СИЛИ (не кінематично)
   - Це дає стабільність + можливість реагувати на удари

═══════════════════════════════════════════════════════════════════════════════
*/

use glam::{Vec3, Quat};
use rapier3d::prelude::*;

use super::{PhysicsWorld, Skeleton, MuscleSystem, BoneId};
use super::muscle::{TargetPose, WalkCycle};
use crate::debug_log::log_debug;

/// Режим роботи ragdoll
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RagdollMode {
    /// Активний контроль - застосовуються сили для руху
    Active,
    /// Ragdoll - м'язи розслаблені, падає під фізикою
    Ragdoll,
    /// Відновлення - поступове повернення контролю
    Recovery { progress: f32 },
}

/// Active Ragdoll персонаж
pub struct ActiveRagdoll {
    /// Фізичний скелет
    pub skeleton: Skeleton,

    /// Система м'язів
    pub muscles: MuscleSystem,

    /// Поточний режим
    pub mode: RagdollMode,

    /// Цикл ходьби
    pub walk_cycle: WalkCycle,

    /// Чи персонаж рухається
    pub is_walking: bool,

    /// Цільовий напрямок руху (world space)
    pub move_direction: Vec3,

    /// Поточна цільова поза
    current_pose: TargetPose,

    // === MOVEMENT CONTROL ===
    /// Цільова позиція (куди хочемо рухатись)
    pub target_position: Vec3,

    /// Цільовий yaw (куди хочемо дивитись)
    pub target_yaw: f32,

    /// Швидкість руху
    pub move_speed: f32,

    /// Сила для утримання вертикального положення
    pub upright_force: f32,

    /// Сила для руху
    pub movement_force: f32,

    /// Сила для обертання
    pub rotation_force: f32,

    /// Лічильник кадрів для логування
    frame_count: u32,
}

impl ActiveRagdoll {
    /// Створює нового персонажа
    pub fn new(physics: &mut PhysicsWorld, position: Vec3) -> Self {
        let skeleton = Skeleton::create_humanoid(physics, position);
        let muscles = MuscleSystem::create_humanoid();

        Self {
            skeleton,
            muscles,
            mode: RagdollMode::Ragdoll,  // Починаємо з чистого ragdoll - тільки гравітація!
            walk_cycle: WalkCycle::new(),
            is_walking: false,
            move_direction: Vec3::NEG_Z,
            current_pose: TargetPose::standing(),
            target_position: position,
            target_yaw: 0.0,
            move_speed: 3.0,
            upright_force: 500.0,
            movement_force: 200.0,
            rotation_force: 100.0,
            frame_count: 0,
        }
    }

    /// Оновлює ragdoll
    pub fn update(&mut self, physics: &mut PhysicsWorld, delta: f32) {
        self.frame_count += 1;

        // Логування кожні 60 кадрів (раз на секунду при 60 FPS)
        if self.frame_count % 60 == 1 {
            self.log_bone_positions(physics);
        }

        // Оновлюємо режим
        match self.mode {
            RagdollMode::Active => {
                self.muscles.global_strength = 1.0;
            }
            RagdollMode::Ragdoll => {
                self.muscles.global_strength = 0.0;
            }
            RagdollMode::Recovery { progress } => {
                let new_progress = (progress + delta * 0.5).min(1.0);
                self.muscles.global_strength = new_progress;

                if new_progress >= 1.0 {
                    self.mode = RagdollMode::Active;
                } else {
                    self.mode = RagdollMode::Recovery { progress: new_progress };
                }
            }
        }

        // Якщо активний режим - застосовуємо контроль
        if self.mode == RagdollMode::Active {
            self.apply_movement_control(physics, delta);
            self.apply_upright_torque(physics);
        }

        // Оновлюємо цикл ходьби
        self.walk_cycle.update(delta, self.is_walking);

        // Генеруємо цільову позу
        if self.is_walking {
            self.current_pose = self.walk_cycle.get_pose();
        } else {
            self.current_pose = TargetPose::standing();
        }

        // Застосовуємо позу до м'язів
        self.muscles.set_pose(&self.current_pose);

        // Оновлюємо м'язи (застосовуємо torque до кінцівок)
        self.muscles.update(physics, &self.skeleton);
    }

    /// Застосовує сили для руху pelvis
    fn apply_movement_control(&mut self, physics: &mut PhysicsWorld, delta: f32) {
        if let Some(handle) = self.skeleton.bodies.get(&BoneId::Pelvis) {
            if let Some(body) = physics.rigid_body_set.get_mut(*handle) {
                // === ГОРИЗОНТАЛЬНИЙ РУХ ===
                if self.is_walking {
                    // Оновлюємо target_position в напрямку руху
                    self.target_position += self.move_direction * self.move_speed * delta;

                    // Поточна позиція
                    let current_pos = Vec3::new(
                        body.translation().x,
                        body.translation().y,
                        body.translation().z,
                    );

                    // Різниця позицій (тільки XZ)
                    let diff = Vec3::new(
                        self.target_position.x - current_pos.x,
                        0.0,
                        self.target_position.z - current_pos.z,
                    );

                    // Застосовуємо силу в напрямку руху
                    let force = diff * self.movement_force;
                    body.add_force(vector![force.x, 0.0, force.z], true);
                }

                // === ОБЕРТАННЯ (YAW) ===
                // Цільова ротація
                let target_quat = Quat::from_rotation_y(self.target_yaw);

                // Поточна ротація
                let current_rot = body.rotation();
                let current_quat = Quat::from_xyzw(
                    current_rot.i,
                    current_rot.j,
                    current_rot.k,
                    current_rot.w,
                );

                // Різниця ротацій
                let error_quat = target_quat * current_quat.inverse();
                let (axis, angle) = error_quat.to_axis_angle();

                // Нормалізуємо кут до [-PI, PI]
                let angle = if angle > std::f32::consts::PI {
                    angle - std::f32::consts::TAU
                } else {
                    angle
                };

                // Torque для повороту (тільки Y)
                let torque_y = axis.y * angle * self.rotation_force;
                let torque_y = torque_y.clamp(-50.0, 50.0);

                body.add_torque(vector![0.0, torque_y, 0.0], true);
            }
        }
    }

    /// Застосовує torque для утримання вертикального положення
    fn apply_upright_torque(&self, physics: &mut PhysicsWorld) {
        if let Some(handle) = self.skeleton.bodies.get(&BoneId::Pelvis) {
            if let Some(body) = physics.rigid_body_set.get_mut(*handle) {
                // Отримуємо поточну орієнтацію
                let rot = body.rotation();

                // Конвертуємо в Euler angles (приблизно)
                // Для простоти - дивимось на "up" вектор тіла
                let up_local = vector![0.0, 1.0, 0.0];
                let up_world = rot * up_local;

                // Якщо up_world не вертикальний - застосовуємо коригуючий torque
                // Cross product дає вісь обертання
                let target_up = vector![0.0, 1.0, 0.0];
                let correction_axis = up_world.cross(&target_up);

                // Кут відхилення (dot product)
                let dot = up_world.dot(&target_up).clamp(-1.0, 1.0);
                let angle = dot.acos();

                // Застосовуємо torque пропорційно відхиленню
                if angle > 0.01 {
                    let torque = correction_axis * angle * self.upright_force;
                    // Обмежуємо
                    let torque = vector![
                        torque.x.clamp(-100.0, 100.0),
                        0.0, // Не впливаємо на yaw
                        torque.z.clamp(-100.0, 100.0)
                    ];
                    body.add_torque(torque, true);
                }

                // Також застосовуємо torque до spine (chest видалено в оптимізації)
                for bone_id in [BoneId::Spine] {
                    if let Some(spine_handle) = self.skeleton.bodies.get(&bone_id) {
                        if let Some(spine_body) = physics.rigid_body_set.get_mut(*spine_handle) {
                            let spine_rot = spine_body.rotation();
                            let spine_up = spine_rot * up_local;
                            let spine_correction = spine_up.cross(&target_up);
                            let spine_dot = spine_up.dot(&target_up).clamp(-1.0, 1.0);
                            let spine_angle = spine_dot.acos();

                            if spine_angle > 0.01 {
                                let spine_torque = spine_correction * spine_angle * self.upright_force * 0.5;
                                let spine_torque = vector![
                                    spine_torque.x.clamp(-50.0, 50.0),
                                    0.0,
                                    spine_torque.z.clamp(-50.0, 50.0)
                                ];
                                spine_body.add_torque(spine_torque, true);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Встановлює напрямок руху
    pub fn set_move_direction(&mut self, direction: Vec3) {
        if direction.length_squared() > 0.01 {
            self.move_direction = direction.normalize();
            self.is_walking = true;

            // Цільовий yaw = напрямок руху
            self.target_yaw = (-direction.x).atan2(-direction.z);
        } else {
            self.is_walking = false;
        }
    }

    /// Переводить в режим ragdoll
    pub fn go_ragdoll(&mut self) {
        self.mode = RagdollMode::Ragdoll;
        self.is_walking = false;
    }

    /// Починає відновлення після ragdoll
    pub fn start_recovery(&mut self) {
        self.mode = RagdollMode::Recovery { progress: 0.0 };
    }

    /// Отримує позицію персонажа (центр pelvis)
    pub fn get_position(&self, physics: &PhysicsWorld) -> Vec3 {
        self.skeleton.get_bone_position(physics, BoneId::Pelvis)
            .unwrap_or(Vec3::ZERO)
    }

    /// Отримує ротацію персонажа (pelvis)
    pub fn get_rotation(&self, physics: &PhysicsWorld) -> Quat {
        self.skeleton.get_bone_rotation(physics, BoneId::Pelvis)
            .unwrap_or(Quat::IDENTITY)
    }

    /// Застосовує імпульс до конкретної кістки (наприклад, при ударі)
    pub fn apply_impact(&mut self, physics: &mut PhysicsWorld, bone_id: BoneId, impulse: Vec3) {
        if let Some(handle) = self.skeleton.bodies.get(&bone_id) {
            if let Some(body) = physics.rigid_body_set.get_mut(*handle) {
                body.apply_impulse(vector![impulse.x, impulse.y, impulse.z], true);
            }

            // Послаблюємо м'яз в точці удару
            if let Some(muscle) = self.muscles.muscles.get_mut(&bone_id) {
                muscle.strength *= 0.3;
            }
        }
    }

    /// Отримує позиції всіх кісток для рендерингу
    pub fn get_bone_transforms(&self, physics: &PhysicsWorld) -> Vec<(BoneId, Vec3, Quat)> {
        BoneId::all_bones()
            .into_iter()
            .filter_map(|bone_id| {
                let pos = self.skeleton.get_bone_position(physics, bone_id)?;
                let rot = self.skeleton.get_bone_rotation(physics, bone_id)?;
                Some((bone_id, pos, rot))
            })
            .collect()
    }

    /// Логує позиції всіх кісток для діагностики
    fn log_bone_positions(&self, physics: &PhysicsWorld) {
        log_debug(&format!("=== RAGDOLL FRAME {} ===", self.frame_count));

        for bone_id in BoneId::all_bones() {
            if let Some(pos) = self.skeleton.get_bone_position(physics, bone_id) {
                let rot = self.skeleton.get_bone_rotation(physics, bone_id)
                    .unwrap_or(Quat::IDENTITY);

                // Euler angles для читабельності
                let (yaw, pitch, roll) = rot.to_euler(glam::EulerRot::YXZ);

                log_debug(&format!(
                    "{:?}: pos=({:.2}, {:.2}, {:.2}) rot=({:.1}°, {:.1}°, {:.1}°)",
                    bone_id,
                    pos.x, pos.y, pos.z,
                    yaw.to_degrees(), pitch.to_degrees(), roll.to_degrees()
                ));
            } else {
                log_debug(&format!("{:?}: MISSING!", bone_id));
            }
        }

        // Відстані між з'єднаними кістками
        log_debug("--- JOINT DISTANCES ---");
        let check_pairs = [
            (BoneId::Pelvis, BoneId::Spine, "Pelvis-Spine"),
            (BoneId::Spine, BoneId::Head, "Spine-Head"),
            (BoneId::Pelvis, BoneId::LeftUpperLeg, "Pelvis-LUpperLeg"),
            (BoneId::LeftUpperLeg, BoneId::LeftLowerLeg, "LUpperLeg-LLowerLeg"),
            (BoneId::Pelvis, BoneId::RightUpperLeg, "Pelvis-RUpperLeg"),
            (BoneId::RightUpperLeg, BoneId::RightLowerLeg, "RUpperLeg-RLowerLeg"),
            (BoneId::Spine, BoneId::LeftUpperArm, "Spine-LUpperArm"),
            (BoneId::LeftUpperArm, BoneId::LeftLowerArm, "LUpperArm-LLowerArm"),
            (BoneId::Spine, BoneId::RightUpperArm, "Spine-RUpperArm"),
            (BoneId::RightUpperArm, BoneId::RightLowerArm, "RUpperArm-RLowerArm"),
        ];

        for (parent, child, name) in check_pairs {
            if let (Some(p1), Some(p2)) = (
                self.skeleton.get_bone_position(physics, parent),
                self.skeleton.get_bone_position(physics, child),
            ) {
                let dist = (p2 - p1).length();
                log_debug(&format!("{}: {:.3}m", name, dist));
            }
        }

        log_debug("");
    }
}
