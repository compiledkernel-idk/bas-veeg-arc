pub mod ai;
pub mod boss_system;
pub mod character_movesets;
pub mod combos;
pub mod combo_system;
pub mod hitbox;
pub mod hurtbox;
pub mod inputs;
pub mod meter;
pub mod plane_system;
pub mod supers;
pub mod weapons;

// Only export what's actively used
pub use hitbox::Hitbox;
pub use hurtbox::Hurtbox;
pub use inputs::InputManager;
pub use combo_system::{ComboSystem, StyleRank, ComboFinisher};
pub use plane_system::{PlaneSystem, BombPattern};
pub use boss_system::{BossManager, BossType};
pub use character_movesets::{CharacterMoveset, CharacterStats, MoveData, MoveId, SpecialTrait};
// Other modules available but not re-exported to reduce unused warnings
