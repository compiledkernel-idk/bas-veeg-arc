use crate::ecs::comp::*;
use crate::ecs::entity::EntityId;
use crate::ecs::world::World;
use macroquad::prelude::*;

pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let entities: Vec<_> = world.query::<Transform>().map(|(e, _)| e).collect();

        for entity in entities {
            if let Some(velocity) = world.get_component::<Velocity>(entity) {
                let vel = velocity.clone();
                if let Some(transform) = world.get_component_mut::<Transform>(entity) {
                    transform.position += vel.linear * dt;
                    transform.rotation += vel.angular * dt;
                }
            }
        }
    }
}

pub struct PhysicsSystem {
    gravity: Vec2,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            gravity: Vec2::new(0.0, 980.0),
        }
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let entities: Vec<_> = world.query::<PhysicsBody>().map(|(e, _)| e).collect();

        for entity in entities {
            if let Some(body) = world.get_component::<PhysicsBody>(entity) {
                let gravity_force = self.gravity * body.gravity_scale;
                let friction = body.friction;
                drop(body);

                if let Some(velocity) = world.get_component_mut::<Velocity>(entity) {
                    velocity.linear += gravity_force * dt;
                    velocity.linear *= 1.0 - friction * dt;
                }
            }
        }
    }
}

pub struct AnimationSystem;

impl System for AnimationSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let entities: Vec<_> = world
            .query_mut::<AnimationController>()
            .map(|(e, _)| e)
            .collect();

        for entity in entities {
            if let Some(anim) = world.get_component_mut::<AnimationController>(entity) {
                anim.timer += dt;

                if anim.timer >= anim.frame_duration {
                    anim.timer = 0.0;
                    anim.frame += 1;

                    if anim.looping {
                        anim.frame = anim.frame % 8;
                    }
                }
            }
        }
    }
}

pub struct CombatSystem {
    hit_registry: Vec<(u32, u32)>,
    player_attack_multiplier: f32,
}

impl CombatSystem {
    pub fn new() -> Self {
        Self {
            hit_registry: Vec::new(),
            player_attack_multiplier: 1.0,
        }
    }

    pub fn set_player_attack_multiplier(&mut self, multiplier: f32) {
        self.player_attack_multiplier = multiplier.max(0.1);
    }
}

impl System for CombatSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        self.hit_registry.clear();

        let attackers: Vec<_> = world
            .query::<HitboxComponent>()
            .filter(|(_, h)| h.active)
            .map(|(e, _)| e)
            .collect();

        let defenders: Vec<_> = world
            .query::<HurtboxComponent>()
            .filter(|(_, h)| h.active)
            .map(|(e, _)| e)
            .collect();

        for attacker in attackers {
            if let Some(hitbox_comp) = world.get_component::<HitboxComponent>(attacker).cloned() {
                if let Some(attacker_transform) = world.get_component::<Transform>(attacker) {
                    let mut hitbox = hitbox_comp.hitbox.clone();
                    let attacker_pos = attacker_transform.position;

                    let attacker_team =
                        if let Some(fighter) = world.get_component::<Fighter>(attacker) {
                            if fighter.facing < 0.0 {
                                hitbox.offset.x = -hitbox.offset.x;
                            }
                            Some(fighter.team)
                        } else {
                            None
                        };

                    for defender in &defenders {
                        if attacker.as_u32() == defender.as_u32() {
                            continue;
                        }

                        if let (Some(att_team), Some(def_team)) = (
                            attacker_team,
                            world.get_component::<Fighter>(*defender).map(|f| f.team),
                        ) {
                            if att_team.is_allied(def_team) {
                                continue;
                            }
                        }

                        if let Some(hitbox_mut) =
                            world.get_component_mut::<HitboxComponent>(attacker)
                        {
                            if hitbox_mut.hits_registered.contains(&defender.as_u32()) {
                                continue;
                            }
                        }

                        if let Some(hurtbox_comp) =
                            world.get_component::<HurtboxComponent>(*defender)
                        {
                            if let Some(defender_transform) =
                                world.get_component::<Transform>(*defender)
                            {
                                let hurtbox = &hurtbox_comp.hurtbox;
                                let defender_pos = defender_transform.position;

                                if self.check_collision(
                                    attacker_pos + hitbox.offset,
                                    hitbox.size,
                                    defender_pos + hurtbox.offset,
                                    hurtbox.size,
                                ) {
                                    if let Some(hitbox_mut) =
                                        world.get_component_mut::<HitboxComponent>(attacker)
                                    {
                                        hitbox_mut.hits_registered.push(defender.as_u32());
                                    }
                                    self.hit_registry
                                        .push((attacker.as_u32(), defender.as_u32()));
                                }
                            }
                        }
                    }
                }
            }
        }

        for (attacker_id, defender_id) in &self.hit_registry {
            self.apply_damage(world, *attacker_id, *defender_id);
        }
    }
}

impl CombatSystem {
    fn check_collision(&self, pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
        let half1 = size1 * 0.5;
        let half2 = size2 * 0.5;

        let left1 = pos1.x - half1.x;
        let right1 = pos1.x + half1.x;
        let top1 = pos1.y - half1.y;
        let bottom1 = pos1.y + half1.y;

        let left2 = pos2.x - half2.x;
        let right2 = pos2.x + half2.x;
        let top2 = pos2.y - half2.y;
        let bottom2 = pos2.y + half2.y;

        left1 < right2 && right1 > left2 && top1 < bottom2 && bottom1 > top2
    }

    fn apply_damage(&self, world: &mut World, attacker_id: u32, defender_id: u32) {
        let mut damage = 10.0;

        let attacker_entity = EntityId(attacker_id);

        if let Some(fighter) = world.get_component::<Fighter>(attacker_entity) {
            if fighter.character_type == CharacterType::Bas {
                damage *= self.player_attack_multiplier;
            }
        }

        if let (Some(attacker_fighter), Some(defender_fighter)) = (
            world.get_component::<Fighter>(attacker_entity),
            world.get_component::<Fighter>(EntityId(defender_id)),
        ) {
            if attacker_fighter.team.is_allied(defender_fighter.team) {
                return;
            }
        }

        if let Some(health) = world.get_component_mut::<Health>(EntityId(defender_id)) {
            health.current = (health.current - damage).max(0.0);
        }

        if let Some(fighter) = world.get_component_mut::<Fighter>(EntityId(defender_id)) {
            fighter.hitstun = 0.5;
            fighter.state = FighterState::Hitstun;
        }

        let push_dir = if let (Some(attacker_transform), Some(defender_transform)) = (
            world.get_component::<Transform>(attacker_entity),
            world.get_component::<Transform>(EntityId(defender_id)),
        ) {
            let diff = defender_transform.position - attacker_transform.position;
            if diff.x.abs() < 0.01 {
                1.0
            } else {
                diff.x.signum()
            }
        } else {
            1.0
        };

        if let Some(defender_velocity) = world.get_component_mut::<Velocity>(EntityId(defender_id))
        {
            defender_velocity.linear.x += push_dir * 220.0;
            defender_velocity.linear.y -= 40.0;
        }
    }
}

pub struct ParticleSystem;

impl System for ParticleSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let entities: Vec<_> = world.query_mut::<Particle>().map(|(e, _)| e).collect();

        let mut to_destroy = Vec::new();

        for entity in entities {
            let mut should_destroy = false;
            let mut velocity_update = Vec2::ZERO;

            if let Some(particle) = world.get_component_mut::<Particle>(entity) {
                particle.lifetime += dt;
                particle.velocity += particle.acceleration * dt;
                velocity_update = particle.velocity * dt;

                if particle.lifetime >= particle.max_lifetime {
                    should_destroy = true;
                }

                let progress = particle.lifetime / particle.max_lifetime;
                let scale =
                    particle.size_start + (particle.size_end - particle.size_start) * progress;

                if let Some(transform) = world.get_component_mut::<Transform>(entity) {
                    transform.position += velocity_update;
                    transform.scale = Vec2::splat(scale);
                }
            }

            if should_destroy {
                to_destroy.push(entity);
            }
        }

        for entity in to_destroy {
            world.destroy_entity(entity);
        }
    }
}

pub struct AISystem;

impl System for AISystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let entities: Vec<_> = world.query::<AIController>().map(|(e, _)| e).collect();

        for entity in entities {
            let (behavior, mut target, difficulty, ready_to_act) =
                match world.get_component_mut::<AIController>(entity) {
                    Some(ai) => {
                        ai.state_timer += dt;
                        let ready = ai.state_timer >= ai.reaction_delay;
                        if ready {
                            ai.state_timer = 0.0;
                            ai.reaction_delay = (0.28_f32 - ai.difficulty * 0.12_f32).max(0.14_f32)
                                + rand::gen_range(0.0, 0.12);
                        }

                        (ai.behavior.clone(), ai.target_entity, ai.difficulty, ready)
                    }
                    None => continue,
                };

            let team = world
                .get_component::<Fighter>(entity)
                .map(|f| f.team)
                .unwrap_or(Team::Enemy);

            let need_new_target = match target {
                Some(id) => world.get_component::<Transform>(id).is_none(),
                None => true,
            };

            if need_new_target {
                let new_target = match team {
                    Team::Enemy => self.find_player(world),
                    Team::Ally => self.find_nearest_by_team(world, entity, Team::Enemy),
                    Team::Player => None,
                };
                if let Some(ai) = world.get_component_mut::<AIController>(entity) {
                    ai.target_entity = new_target;
                    target = new_target;
                }
            }

            let target = match target {
                Some(id) => id,
                None => continue,
            };

            let ai_pos = match world.get_component::<Transform>(entity) {
                Some(transform) => transform.position,
                None => continue,
            };

            let target_pos = match world.get_component::<Transform>(target) {
                Some(transform) => transform.position,
                None => continue,
            };

            let separation = target_pos - ai_pos;
            let distance = separation.length();

            let (approach_distance, retreat_distance) = self.behavior_distances(&behavior);
            let speed = 160.0 + 100.0 * difficulty;
            let retreat_speed = 120.0 + 80.0 * difficulty;
            let depth_speed = 140.0 + 60.0 * difficulty;

            let mut movement_dir = 0.0;
            let mut movement_speed = 0.0;

            if distance > approach_distance {
                movement_dir = separation.x.signum();
                movement_speed = speed;
            } else if distance < retreat_distance {
                movement_dir = -separation.x.signum();
                movement_speed = retreat_speed;
            }

            let is_moving = movement_dir.abs() > 0.1;
            let depth_difference = target_pos.y - ai_pos.y;

            if let Some(velocity) = world.get_component_mut::<Velocity>(entity) {
                if is_moving {
                    velocity.linear.x = movement_dir.signum() * movement_speed;
                } else {
                    velocity.linear.x = 0.0;
                }

                if depth_difference.abs() > 6.0 {
                    velocity.linear.y = depth_difference.signum() * depth_speed;
                } else {
                    velocity.linear.y = 0.0;
                }
            }

            if let Some(transform) = world.get_component_mut::<Transform>(entity) {
                transform.position.x = transform.position.x.clamp(50.0, screen_width() * 1.2);
                transform.position.y = transform.position.y.clamp(340.0, 660.0);
            }

            if let Some(fighter) = world.get_component_mut::<Fighter>(entity) {
                if fighter.hitstun <= 0.0 && fighter.blockstun <= 0.0 {
                    if is_moving
                        && matches!(fighter.state, FighterState::Idle | FighterState::Walking)
                    {
                        fighter.state = FighterState::Walking;
                    } else if !is_moving && fighter.state == FighterState::Walking {
                        fighter.state = FighterState::Idle;
                    }

                    if separation.x.abs() > 1.0 {
                        fighter.facing = separation.x.signum();
                    }

                    if ready_to_act {
                        if let Some(new_state) = self.choose_action(&behavior, distance) {
                            fighter.state = new_state;
                        }
                    }
                }
            }
        }
    }
}

impl AISystem {
    fn find_player(&self, world: &World) -> Option<crate::ecs::entity::EntityId> {
        world
            .query::<Fighter>()
            .find(|(_, fighter)| fighter.character_type == CharacterType::Bas)
            .map(|(entity, _)| entity)
    }

    fn find_nearest_by_team(
        &self,
        world: &World,
        origin: crate::ecs::entity::EntityId,
        team: Team,
    ) -> Option<crate::ecs::entity::EntityId> {
        let origin_pos = world
            .get_component::<Transform>(origin)
            .map(|t| t.position)?;

        let mut closest = None;
        let mut min_dist_sq = f32::MAX;

        for (entity, fighter) in world.query::<Fighter>() {
            if entity == origin || fighter.team != team {
                continue;
            }

            if let Some(transform) = world.get_component::<Transform>(entity) {
                let dist_sq = (transform.position - origin_pos).length_squared();
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    closest = Some(entity);
                }
            }
        }

        closest
    }

    fn behavior_distances(&self, behavior: &AIBehavior) -> (f32, f32) {
        match behavior {
            AIBehavior::Aggressive => (110.0, 35.0),
            AIBehavior::Defensive => (170.0, 90.0),
            AIBehavior::Balanced => (140.0, 60.0),
            AIBehavior::Support => (130.0, 45.0),
            AIBehavior::Evasive => (200.0, 110.0),
            AIBehavior::Boss(_) => (150.0, 50.0),
        }
    }

    fn choose_action(&self, behavior: &AIBehavior, distance: f32) -> Option<FighterState> {
        let roll = rand::gen_range(0.0, 1.0);

        match behavior {
            AIBehavior::Aggressive => {
                if distance < 70.0 {
                    if roll < 0.7 {
                        Some(FighterState::HeavyAttack)
                    } else {
                        Some(FighterState::LightAttack)
                    }
                } else if distance < 140.0 {
                    if roll < 0.5 {
                        Some(FighterState::LightAttack)
                    } else {
                        Some(FighterState::Special)
                    }
                } else {
                    None
                }
            }
            AIBehavior::Defensive => {
                if distance < 90.0 {
                    if roll < 0.5 {
                        Some(FighterState::Blocking)
                    } else {
                        Some(FighterState::LightAttack)
                    }
                } else if distance < 150.0 {
                    Some(FighterState::HeavyAttack)
                } else {
                    None
                }
            }
            AIBehavior::Balanced => {
                if distance < 80.0 {
                    if roll < 0.4 {
                        Some(FighterState::LightAttack)
                    } else if roll < 0.7 {
                        Some(FighterState::HeavyAttack)
                    } else {
                        Some(FighterState::Blocking)
                    }
                } else if distance < 160.0 {
                    if roll < 0.6 {
                        Some(FighterState::LightAttack)
                    } else {
                        Some(FighterState::Special)
                    }
                } else {
                    None
                }
            }
            AIBehavior::Support => {
                if distance < 90.0 {
                    if roll < 0.5 {
                        Some(FighterState::LightAttack)
                    } else if roll < 0.8 {
                        Some(FighterState::HeavyAttack)
                    } else {
                        Some(FighterState::Special)
                    }
                } else if distance < 180.0 {
                    if roll < 0.7 {
                        Some(FighterState::LightAttack)
                    } else {
                        Some(FighterState::HeavyAttack)
                    }
                } else {
                    None
                }
            }
            AIBehavior::Evasive => {
                if distance < 100.0 {
                    if roll < 0.5 {
                        Some(FighterState::Dodging)
                    } else {
                        Some(FighterState::Blocking)
                    }
                } else {
                    None
                }
            }
            AIBehavior::Boss(phase) => match phase {
                BossPhase::Phase1 => {
                    if distance < 120.0 {
                        Some(FighterState::HeavyAttack)
                    } else {
                        None
                    }
                }
                BossPhase::Phase2 => {
                    if distance < 100.0 {
                        Some(FighterState::Special)
                    } else if distance < 180.0 {
                        Some(FighterState::LightAttack)
                    } else {
                        None
                    }
                }
                BossPhase::Phase3 => {
                    if roll < 0.5 {
                        Some(FighterState::Super)
                    } else if distance < 150.0 {
                        Some(FighterState::HeavyAttack)
                    } else {
                        None
                    }
                }
            },
        }
    }
}
