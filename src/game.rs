use bevy::{prelude::*};

use crate::{board::{BoardPosition, Team, Board}, AppState};

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
    EvaluatingEndConditions,
    EndConditionMet(EndState),
    Paused,
}

/// Occurs right after changing NextState to TakingTurn.
#[derive(Event)]
pub struct NextTurnBeginEvent;

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
        app.add_event::<NextTurnBeginEvent>()
            .add_event::<TurnEndedEvent>()
            .insert_state(GameState::default())
            .add_systems(
                OnEnter(AppState::RunningGame(GameState::EvaluatingEndConditions)),
                evaluate_end_condition_system
            )
            // .add_systems(
            //     Update,
            //     evaluate_end_condition_system
            //         .run_if(in_state(AppState::RunningGame(GameState::EvaluatingEndConditions))),
            // )
            // .add_systems(
            //     Update,
            //     end_turn_system
            //         .run_if(in_state(AppState::RunningGame(GameState::TakingTurn))),
            // )
        ;
    }
}

pub enum WinCondition {
    Won(Team),
    Draw,
}

/// Remove TakingTurnMarker and emit a "TurnEnded" event for the Entity
/// currently marked by TakingTurnMarker.
pub fn end_turn_system(
    mut commands: Commands,
    mut turn_ended_e: EventWriter<TurnEndedEvent>,
    query: Query<(Entity, &TakingTurnMarker)>,
    mut next_state: NextState<GameState>,
) {
    let (just_ended, marker) = query.single();
    debug!("Removing TakingTurnMarker {:?} from {:?}", marker, just_ended);

    commands.entity(just_ended).remove::<TakingTurnMarker>();
    turn_ended_e.send(TurnEndedEvent(just_ended));
}


/// Evaluate the board to check game-end conditions.
///
/// Will read the position of the last piece placed, and the team that placed it
/// to simplify the algorithm for checking.
///
/// Set `NextState` to EndConditionMet(EndState::{condition met}) if an end condition is met.
/// Otherwise, set `NextState` to TakingTurn.
pub fn evaluate_end_condition_system(
    mut commands: Commands,
    board: Res<Board>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // TODO - Actually implement this
    if false {
        debug!("No end condition met, moving to next turn");
        next_state.set(GameState::TakingTurn);
    } else {

    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Based on example code from https://github.com/bevyengine/bevy/blob/main/tests/how_to_test_systems.rs
    #[test]
    fn test_evaluate_end_condition_system() {
        let mut app = App::new();

        // Pre-requisites
        app.insert_state(AppState::RunningGame(GameState::EvaluatingEndConditions));
        app.add_plugins(TurnBasedGameplayPlugin);

        let state: State<AppState> = State::from_world(&mut app.world);
        todo!("Figure out correct syntax for testing this");
        let next_state: NextState<AppState> = NextState::from_world(&mut app.world);
        assert_eq!(*state.get(), AppState::RunningGame(GameState::EvaluatingEndConditions));

        // Tick to trigger systems that run on Update schedule
        app.update();

        // State should be set to TakingTurn now
        let s: State<AppState> = State::from_world(&mut app.world);
        let next_state: NextState<AppState> = NextState::from_world(&mut app.world);
        assert_eq!(*state.get(), AppState::RunningGame(GameState::TakingTurn));

        // TODO - Once actually implemented, add more to this test
    }
}

