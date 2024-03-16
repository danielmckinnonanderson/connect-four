use bevy::prelude::*;

use crate::board::{BoardPosition, Team, Board};

#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndState {
    Won((Entity, [BoardPosition; 4])),
    Draw
}

#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Initializing,
    TakingTurn,
    EvaluatingEndConditions(BoardPosition),
    EndConditionMet(EndState),
    Paused,
}

#[derive(Event)]
pub struct TurnBeginEvent(Entity);

/// Other modules can listen for TurnEndEvent in order to
/// check end conditions before advancing to the next turn.
#[derive(Event)]
pub struct TurnEndedEvent(Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TakingTurnMarker;

/// Defines the ability to be represented in the turn order.
/// Contains a value .0 of type `usize`, which is the value
/// of this Entity in the turn order
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnTaker(usize);

pub struct TurnBasedGameplayPlugin;

impl Plugin for TurnBasedGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnBeginEvent>()
            .add_event::<TurnEndedEvent>()
            .insert_state(GameState::default());
            // .add_systems(
            //     Update,
            //     evaluate_win_condition_system
            //         .run_if(in_state(GameState::EvaluatingEndConditions()
            // )));
    }
}

pub enum WinCondition {
    Won(Team),
    Draw,
}

pub type WinConditionEvaluator = fn (
    board: Board,
    evaluating: Team,
    just_placed: BoardPosition,
) -> Option<WinCondition>;

/// Remove TakingTurnMarker and emit a "TurnEnded" event for the Entity
/// currently marked by TakingTurnMarker.
pub fn end_turn_system(
    mut commands: Commands,
    mut turn_ended_e: EventWriter<TurnEndedEvent>,
    query: Query<(Entity, &TakingTurnMarker)>
) {
    let (just_ended, marker) = query.single();
    debug!("Removing component {:?} from {:?}", marker, just_ended);

    commands.entity(just_ended).remove::<TakingTurnMarker>();
    turn_ended_e.send(TurnEndedEvent(just_ended));
}

pub fn get_next_turn(
    just_went: Entity,
    query: Query<(Entity, &TurnTaker)>
) {
    for (entity, turn_order_value) in query.into_iter() {
        if entity == just_went {
            
        }
    }
}

pub fn evaluate_win_condition_system(
    mut commands: Commands,
) {

}

