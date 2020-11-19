use crate::unit::{UnitAction};




pub struct PawnMoveAction;


impl UnitAction for PawnMoveAction {
    fn list_targets(
        entity: Entity,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> dyn Iterator<Item = Position> {
        unimplmeneted!()
    }

    fn execute(
        entity: Entity,
        target: Position,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> dyn Iterator<Item = (ObjectId, unit::events::UnitCmd)> {
        unimplmeneted!()
    }
}