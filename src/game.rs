use bevy::{prelude::*};

use crate::{board::{BoardPosition, Team, Board}, AppState, TEXT_COLOR};

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreenMarker;

pub fn game_setup(
    mut _commands: Commands,
    mut next_game_state: ResMut<NextState<AppState>>
) {
    debug!("Running new game setup...");
    next_game_state.set(AppState::WaitForSelection);
    debug!("Done setting up new game!");
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnNumber(u16);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TakingTurnMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerMarker;

/// Component added to an entity when they have won the game.
/// Contains an array of four board positions, which is their
///  winning combination.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinnerMarker(pub [BoardPosition; 4]);

/// Evaluate the board to check game-end conditions.
///
/// Will read the position of the last piece placed, and the team that placed it
/// to simplify the algorithm for checking.
///
/// Set `NextState` to EndConditionMet(EndState::{condition met}) if an end condition is met.
/// Otherwise, set `NextState` to TakingTurn.
pub fn evaluate_end_condition_system(
    mut commands: Commands,
    turn_number: ResMut<TurnNumber>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<AppState>>,
    just_took_turn_query: Query<Entity, (With<TakingTurnMarker>, With<PlayerMarker>)>,
) {
    debug!("Evaluating board state to check end conditions...");

    if turn_number.0 < 4 {
        debug!("Turn number is less than 4, skipping end condition check");
        next_state.set(AppState::WaitForSelection);
        return;
    }

    // TODO - Actually implement this
    if false {
        debug!("No end condition met, moving to next turn");
        next_state.set(AppState::WaitForSelection);
    } else {
        debug!("End condition met, moving to terminal state");
        next_state.set(AppState::Winner);
        let entity = just_took_turn_query.single();
        commands.entity(entity).insert(
            WinnerMarker([BoardPosition::try_from((0, 0)).expect("This never should've even made it to the debug build"); 4])
        );
    }
}

/// Invoked when the end conditions are not satisfied.
/// In other words, when the state is transitioning from
/// checking the end conditions to the next turn.
///
/// Increment the turn number, and move the TakingTurnMarker
/// struct from one player to the other.
pub fn advance_to_next_turn_system(
    mut commands: Commands,
    turn_number: ResMut<TurnNumber>,
    taking_turn_query: Query<(Entity, Option<&TakingTurnMarker>), With<PlayerMarker>>
) {
    // TODO - Surely there is a better way to do this. 
    let mut len = 0;

    for player in &taking_turn_query {
        if let Some(_taking_turn_marker) = player.1 {
            // Remove the taking turn marker component from this entity.
            commands.entity(player.0).remove::<TakingTurnMarker>();
            debug!("Removed TakingTurnMarker from entity {:?}", player.0);
        } else {
            // Add the taking turn marker component to this entity.
            commands.entity(player.0).insert(TakingTurnMarker);
            debug!("Added TakingTurnMarker to entity {:?}", player.0);
        }
        len += 1;
    }
    debug_assert!(
        len == 2,
        "Tried to advance to the next turn, but there are more than two players!"
    );

    let turn_number = turn_number.into_inner();
    turn_number.0 += 1;
    debug!("Ready to proceed to turn number {:?}", turn_number);
}

#[cfg(test)]
mod test {
    use super::*;

    // Based on example code from https://github.com/bevyengine/bevy/blob/main/tests/how_to_test_systems.rs
    #[test]
    fn test_evaluate_end_condition_system() {
        let mut app = App::new();

        // Pre-requisites
        app.insert_state(AppState::EvaluateBoard);
        let state: State<AppState> = State::from_world(&mut app.world);
        assert_eq!(*state.get(), AppState::EvaluateBoard);

        // Tick to trigger systems that run on Update schedule
        app.update();

        // State should be set to TakingTurn now
        let state: State<AppState> = State::from_world(&mut app.world);
        let next_state: NextState<AppState> = NextState::from_world(&mut app.world);
        assert_eq!(*state.get(), AppState::WaitForSelection);

        // TODO - Once actually implemented, add more to this test
    }
}

