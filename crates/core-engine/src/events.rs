use crate::components::MapPosition;

use super::resources::world::MapId;

pub struct ShouldUpdateMap(pub MapId);

pub struct ShouldSendFullMapUpdateToClient(pub MapId);

pub struct TryAttack {
    pub map_position: MapPosition,
    pub attack_value: i32,
}
