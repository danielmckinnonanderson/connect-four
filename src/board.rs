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
    pub team: Team,
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
    // _query: Query<(&TeamComponent, &TakingTurnMarker)>,
) {
    debug!("Running insert_piece_system");
    let mut ctr = 0;

    for event in insert_piece_event.read() {
        debug!("Received InsertPieceEvent for column {}", event.column);
        if let Ok(()) = push_piece_to_column(&mut board, Team::A, event.column) {
            debug!("Inserted piece into column {}", event.column);
        } else {
            error!("Could not insert piece into column {}", event.column);
        }
        ctr += 1;
    }

    if ctr > 1 {
        error!("insert_piece_system ran more than once in a single frame, processed {} InsertPieceEvents", ctr);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn testutil_create_input_event_system(
        mut event_writer: EventWriter<InsertPieceEvent> 
    ) {
        println!("Writing test event");
        event_writer.send(InsertPieceEvent {
            team: Team::A,
            column: 1,
        });
    }

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

    // Based on example code from https://github.com/bevyengine/bevy/blob/main/tests/how_to_test_systems.rs
    #[test]
    fn test_insert_piece_system() {
        let mut app = App::new();

        // Pre-requisites
        app.insert_resource(Board::new());
        app.insert_state(AppState::RunningGame(GameState::TakingTurn));
        app.add_event::<InsertPieceEvent>();
        // Guarantee that we create an input event before checking run 
        // conditions for insert_piece system
        app.add_systems(Update, (
                testutil_create_input_event_system,
                insert_piece_system,
            ).chain());

        let _player_id = app.world.spawn((
            TeamComponent(Team::A),
            TakingTurnMarker {})
        ).id();

        // Tick to trigger systems that run on Update schedule
        app.update();

        // Get EventReader
        let insert_piece_event = app.world.resource::<Events<InsertPieceEvent>>();
        let mut insert_piece_reader = insert_piece_event.get_reader();

        let insert_piece = insert_piece_reader.read(insert_piece_event).next().unwrap();

        assert_eq!(insert_piece.column, 1);
        let board = app.world.resource::<Board>();
        assert_eq!(board.board.get(&BoardPosition(1, 1)), Some(&Some(Team::A)));
    }
}

