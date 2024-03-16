use bevy::{prelude::*, utils::hashbrown::HashMap, render::render_resource::encase::vector::FromVectorParts};

pub const BOARD_HEIGHT: u8 = 7;
pub const BOARD_WIDTH: u8 = 9;

/// Players will be associated with a team.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Team {
    A,
    B, 
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
        for x in 1..BOARD_WIDTH {
            for y in 1..BOARD_HEIGHT {
                board.insert(BoardPosition(x, y), None);
            }
        }

        Board { board }
    }
}

pub struct ConnectFourBoardPlugin;

impl Plugin for ConnectFourBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new());
    }
}


impl Board {
    pub fn has_position(&self, position: BoardPosition) -> bool {
        self.board.contains_key(&position)
    }
    
    pub fn push_piece_to_column(&mut self, piece: Team, column: u8) -> Result<(), ()> {
        if let Some((empty_space, _)) = self.board
            .iter()
            .min_by_key(|pair: &(&BoardPosition, &Option<Team>)| {
                println!("Checking {:?}", pair);
                // Find the BoardPosition which has the highest y / .1 value,
                //  whose Option<TeampComponent> is None (the position is unoccupied)
                if pair.0.0 == column && pair.1.is_none() {
                    Some(pair.0.1)
                } else {
                    None
                }
            }) {

            // Push this team's piece to the empty space we identified
            debug!("Pushed piece to column {}, now occupying space {:?}", column, empty_space);
            self.board.insert(*empty_space, Some(piece));
            Ok(())
        } else {
            // Error just logs a message for now, in theory this should be illegal
            debug!("Could not push piece to column {}, column was full or does not exist", column);
            // error!("Could not push piece to column {}, column was full or does not exist", column);
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(board.board.len(), 42);
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

        let position = BoardPosition(1, 1);

        let result = b.push_piece_to_column(Team::A, 1);
        assert_eq!(result, Ok(()));
        assert_eq!(b.board.get(&position), Some(&Some(Team::A)));

        let result = b.push_piece_to_column(Team::B, 1);
        assert_eq!(result, Err(()));
        assert_eq!(b.board.get(&position), Some(&Some(Team::A)));
    }
}

