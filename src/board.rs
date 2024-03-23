use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::game::OnGameScreenMarker;

pub const BOARD_HEIGHT: u8 = 7;
pub const BOARD_WIDTH: u8 = 9;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BoardX {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl TryFrom<usize> for BoardX {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            8 => Ok(Self::I),
            _ => Err(())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BoardY {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl TryFrom<usize> for BoardY {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Three),
            3 => Ok(Self::Four),
            4 => Ok(Self::Five),
            5 => Ok(Self::Six),
            6 => Ok(Self::Seven),
            _ => Err(())
        }
    }
}

/// Players will be associated with a team.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Team {
    A,
    B, 
}

#[derive(Component, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TeamComponent(Team);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BoardPosition(pub BoardX, pub BoardY);

impl TryFrom<(u8, u8)> for BoardPosition {
    type Error = ();

    fn try_from((x, y): (u8, u8)) -> Result<Self, Self::Error> {
        let try_x = BoardX::try_from(x as usize);
        let try_y = BoardY::try_from(y as usize);

        if let (Ok(x), Ok(y)) = (try_x, try_y) {
            Ok(BoardPosition(x, y))
        } else {
            Err(())
        }
    }
}

/// Each ConnectFourPiece Entity will have a board position
#[derive(Component)]
pub struct BoardPositionComponent(BoardPosition);

#[derive(Resource)]
pub struct Board(HashMap<BoardPosition, Option<Team>>);

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = HashMap::new();
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                board.insert(
                    BoardPosition(
                        BoardX::try_from(x as usize).expect("You have fucked up."),
                        BoardY::try_from(y as usize).expect("You have fucked up.")),
                    None);
            }
        }

        Board(board)
    }
}

pub fn draw_board_system(
    mut commands: Commands,
) {
    let root_entity = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    }).id();


    for _ in 0..BOARD_WIDTH {
        let column_entity = commands.spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(11.111), // 100.0 / 9 columns
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        },
            OnGameScreenMarker
        )).id();

        commands.entity(root_entity).push_children(&[column_entity]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(board.0.len(), 63);
        assert_eq!(board.0.get(&BoardPosition::try_from((0, 0)).unwrap()), None);
        assert_eq!(board.0.get(&BoardPosition::try_from((1, 1)).unwrap()), Some(&None));
    }
}

