use bevy::{prelude::*, utils::hashbrown::HashMap, ecs::event::event_update_condition};

use crate::{AppState, game::{TakingTurnMarker, GameState}};

pub const BOARD_HEIGHT: u8 = 7;
pub const BOARD_WIDTH: u8 = 9;

/// Players will be associated with a team.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Team {
    A,
    B, 
}

#[derive(Event, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct InsertPieceEvent {
    pub column: u8,
}

#[derive(Component, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TeamComponent(Team);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BoardPosition(pub u8, pub u8);

/// Each ConnectFourPiece Entity will have a board position
#[derive(Component)]
pub struct BoardPositionComponent(BoardPosition);

#[derive(Resource)]
pub struct Board {
    pub board: HashMap<BoardPosition, Option<Team>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = HashMap::new();
        for x in 1..BOARD_WIDTH + 1 {
            for y in 1..BOARD_HEIGHT + 1 {
                board.insert(BoardPosition(x, y), None);
            }
        }

        Board { board }
    }
}

pub struct ConnectFourBoardPlugin;

impl Plugin for ConnectFourBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new())
            .add_event::<InsertPieceEvent>()
            .add_systems(Update,
                insert_piece_system
                .run_if(in_state(AppState::RunningGame(GameState::TakingTurn)))
            )
        ;
    }
}

fn push_piece_to_column(board_ref: &mut Board, piece: Team, column: u8) -> Result<(), ()> {
    debug!("Pushing piece to column {}", column);
    if let Some((empty_space, _)) = board_ref.board
        .iter()
        .filter(|pair| {
            pair.0.0 == column && pair.1.is_none()
        })
        .min_by_key(|pair| {
            // Find the BoardPosition which has the highest y / .1 value,
            //  whose Option<TeampComponent> is None (the position is unoccupied)
            Some(pair.0.1)
        }) {

        // Push this team's piece to the empty space we identified
        debug!("Pushed piece to column {}, now occupying space {:?}", column, empty_space);
        board_ref.board.insert(*empty_space, Some(piece));
        Ok(())
    } else {
        // Error just logs a message for now, in theory this should be illegal
        error!("Could not push piece to column {}, column was full or does not exist", column);
        Err(())
    }
}


impl Board {
    pub fn has_position(&self, position: BoardPosition) -> bool {
        self.board.contains_key(&position)
    }
}

/// When an InsertPieceEvent is received, we will insert a piece into the board
/// for the team of the Entity currently taking their turn (marked by TakingTurnMarker).
///
/// This system should only run if the game is in the TakingTurn state.
pub fn insert_piece_system(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut insert_piece_event: EventReader<InsertPieceEvent>,
    _state: Res<State<AppState>>,
    _query: Query<(&TeamComponent, &TakingTurnMarker)>,
) {
    let mut ctr = 0;

    for event in insert_piece_event.read() {
        debug!("Received InsertPieceEvent for column {}", event.column);
        if let Ok(()) = push_piece_to_column(&mut board, Team::A, event.column) {
            debug!("Inserted piece into column {}", event.column);
            commands.spawn(TeamComponent(Team::A));
        } else {
            error!("Could not insert piece into column {}", event.column);
        }
        ctr += 1;
    }

    debug_assert!(ctr <= 1,
        "insert_piece_system ran more than once in a single frame, processed {} InsertPieceEvents", ctr);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(board.board.len(), 63);
        assert_eq!(board.board.get(&BoardPosition(0, 0)), None);
        assert_eq!(board.board.get(&BoardPosition(1, 1)), Some(&None));
    }

    #[test]
    fn test_has_position() {
        let mut board = Board {
            board: HashMap::new(),
        };

        let position = BoardPosition(0, 0);
        assert_eq!(board.has_position(position), false);

        board.board.insert(position, None);
        assert_eq!(board.has_position(position), true);
    }

    #[test]
    fn test_push_piece_to_column() {
        let mut b = Board::new();

        for i in 1..BOARD_HEIGHT + 1 {
            let result = push_piece_to_column(&mut b, Team::A, 1);
            assert_eq!(result, Ok(()));
            assert_eq!(b.board.get(&BoardPosition(1, i)), Some(&Some(Team::A)));
        }

        let result = push_piece_to_column(&mut b, Team::B, 1);
        assert_eq!(result, Err(()));
    }
}

