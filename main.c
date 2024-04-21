#include <stdio.h>
#include <stdlib.h>
#include "raylib.h"
#include "raymath.h"

#define WINDOW_HEIGHT 720
#define WINDOW_WIDTH 720
#define WINDOW_TITLE "Wonderful World"
#define TARGET_FPS 60

#define BOARD_HEIGHT 8
#define BOARD_WIDTH 8

#define BOARD_CELL_SIZE WINDOW_HEIGHT / BOARD_HEIGHT

#define COLOR_A RED
#define COLOR_A_BG MAROON // TODO - Make this darker

#define COLOR_B BLUE
#define COLOR_B_BG DARKBLUE // TODO - Make this darker

#define PLAYER_A_VALUE 1
#define PLAYER_B_VALUE 2

typedef enum BoardOccupant {
    EMPTY = 0,
    PLAYER_A = PLAYER_A_VALUE,
    PLAYER_B = PLAYER_B_VALUE,
} BoardOccupant;

typedef enum State {
    INIT = 0,
    TAKING_TURN_A = 1,
    TAKING_TURN_B = 2,
    WINNER_A = 3,
    WINNER_B = 4,
    DRAW = 5,
} State;

typedef struct GameHistory {
    int team_placed; // Value of PLAYER_A_VALUE or PLAYER_B_VALUE
    int row; // Location of the piece
    int col;
    struct GameHistory *previous; // Or NULL if this is the first move
} GameHistory;

typedef struct GameState {
    State state;
    int board[BOARD_HEIGHT][BOARD_WIDTH];
    int board_height;
    int board_width;
    GameHistory *history; // Linked list of previous moves
} GameState;


void UpdateGame(GameState *game);
void DrawGame(const GameState *game, const Vector2 *mouse_pos);
int LowestOpenSpace(const GameState *game, const int col);
State EvaluateBoard(const GameState *game);


int main(void)
{
    // Setup the window
    InitWindow(
        WINDOW_WIDTH, 
        WINDOW_HEIGHT,
        WINDOW_TITLE
    );

    SetTargetFPS(TARGET_FPS);

    GameState game = {
        .state = INIT,
        .board_height = BOARD_HEIGHT,
        .board_width = BOARD_WIDTH,
        .board = {},
    };

    // Initialize the game state
    for (int i = 0; i < BOARD_HEIGHT; i++) {
        for (int j = 0; j < BOARD_WIDTH; j++) {
            game.board[i][j] = EMPTY;
        }
    }

    // Go!
    while (!WindowShouldClose())
    {
        // Update
        UpdateGame(&game);

        // Draw
        BeginDrawing();
        {
            Vector2 mouse_pos = GetMousePosition();
            DrawGame(&game, &mouse_pos);
        }
        EndDrawing();
    }

    // Cleanup
    CloseWindow();
    return 0;
}

void UpdatePlayerSelection(GameState *game,
                           const int col,
                           const int lowest_open_row,
                           const int player)
{
    // Column has an open space, place the piece
    game->board[lowest_open_row][col] = player;

    // Update history
    // TODO - Free history on teardown
    GameHistory *new_history = (GameHistory *) malloc(sizeof(GameHistory));
    new_history->team_placed = player == PLAYER_A ? PLAYER_A_VALUE : PLAYER_B_VALUE;
    new_history->row = lowest_open_row;
    new_history->col = col;
    new_history->previous = game->history;
    game->history = new_history;
}

void UpdateGame(GameState *game)
{
    // Update
    Vector2 mouse_pos = GetMousePosition();
    switch (game->state)
    {
        case INIT:
            // Initialize the game state
            game->state = TAKING_TURN_A;
            break;
        case TAKING_TURN_A:
            if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
                printf("Mouse position: %f, %f\n", mouse_pos.x, mouse_pos.y);
                int col = (int) (mouse_pos.x / ((float)BOARD_CELL_SIZE));
                printf("Col: %d\n", col);

                int lowest_open_row = LowestOpenSpace(game, col);
                if (lowest_open_row == -1) {
                    // Column is full
                    printf("Column %d is full\n", col);
                    break;
                }

                // Column has an open space, place the piece & update the history
                UpdatePlayerSelection(game, col, lowest_open_row, PLAYER_A);

                // Evaluate the board and advance to next state
                game->state = EvaluateBoard(game);
            }
            break;
        case TAKING_TURN_B:
            if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
                printf("Mouse position: %f, %f\n", mouse_pos.x, mouse_pos.y);
                int col = (int) (mouse_pos.x / ((float)BOARD_CELL_SIZE));
                printf("Col: %d\n", col);

                int lowest_open_row = LowestOpenSpace(game, col);
                if (lowest_open_row == -1) {
                    // Column is full
                    printf("Column %d is full\n", col);
                    break;
                }

                // Column has an open space, place the piece & update the history
                UpdatePlayerSelection(game, col, lowest_open_row, PLAYER_B);

                // Advance to next state
                game->state = EvaluateBoard(game);
            }
            break;
        case WINNER_A:
            break;
        case WINNER_B:
            break;
        case DRAW:
            break;
        default:
            printf("Invalid game state: %d\n", game->state);
            exit(1);
            break;
    }
}

void DrawGridlines(const GameState *game)
{
    ClearBackground(BLACK);

    // Draw horizontal and veritcal lines at the boundaries of each cell
    for (int i = 0; i < game->board_height; i++) {
        // Draw boundary line
        DrawLine(
            0, 
            i * BOARD_CELL_SIZE, 
            WINDOW_WIDTH, 
            i * BOARD_CELL_SIZE, 
            WHITE
        );
    }
    for (int i = 0; i < game->board_width; i++) {
        DrawLine(
            i * BOARD_CELL_SIZE, 
            0, 
            i * BOARD_CELL_SIZE, 
            WINDOW_HEIGHT, 
            WHITE
        );
    }
}

void DrawGame(const GameState *game, const Vector2 *mouse_pos)
{
    // If mouse position is within column, highlight entire column
    int col = (int) (mouse_pos->x / ((float)BOARD_CELL_SIZE));

    // Highlight the column to indicate where the current player is potentially placing
    if (game->state == TAKING_TURN_A || game->state == TAKING_TURN_B) {
        Color team_bg_highlight = game->state == TAKING_TURN_A ? MAROON : DARKBLUE;
        DrawRectangle(
            col * BOARD_CELL_SIZE, 
            0, 
            BOARD_CELL_SIZE, 
            WINDOW_HEIGHT, 
            team_bg_highlight
        );
    }

    // Draw the gridlines on top of the highlighted column
    DrawGridlines(game);

    // Fill in the squares of the board grid with nothing if empty,
    // red if team A, and blue if team B
    for (int i = 0; i < game->board_height; i++) {
        for (int j = 0; j < game->board_width; j++) {
            if (game->board[i][j] == PLAYER_A) {
                DrawRectangle(
                    j * BOARD_CELL_SIZE, 
                    i * BOARD_CELL_SIZE, 
                    BOARD_CELL_SIZE, 
                    BOARD_CELL_SIZE, 
                    COLOR_A
                );
            } else if (game->board[i][j] == PLAYER_B) {
                DrawRectangle(
                    j * BOARD_CELL_SIZE, 
                    i * BOARD_CELL_SIZE, 
                    BOARD_CELL_SIZE, 
                    BOARD_CELL_SIZE, 
                    COLOR_B
                );
            }

            // Always draw some text within the upper left corner
            // of the cell with the cell's coordinates
            DrawText(
                TextFormat("%d, %d", i, j),
                j * BOARD_CELL_SIZE + 5,
                i * BOARD_CELL_SIZE + 5,
                10,
                WHITE
            );
        }
    }

    // If there is a winner, show text saying "Player X wins!"
    // If there is a draw, show text saying "Draw!"
    switch (game->state)
    {
        case WINNER_A:
            // Large, centered text saying "Player A wins!"
            DrawText(
                "Player A wins!",
                WINDOW_WIDTH / 2 - 224,
                WINDOW_HEIGHT / 2 - 10,
                64,
                COLOR_A_BG
            );
            break;
        case WINNER_B:
            // Large, centered text saying "Player B wins!"
            DrawText(
                "Player B wins!",
                WINDOW_WIDTH / 2 - 224,
                WINDOW_HEIGHT / 2 - 10,
                64,
                COLOR_B_BG
            );
            break;
        case DRAW:
            DrawText(
                "Draw!",
                WINDOW_WIDTH / 2 - 72,
                WINDOW_HEIGHT / 2 - 10,
                64,
                BLACK 
            );
            break;
        default:
            break;
    }
}

// Walk the column, returning the lowest (highest actual int value) row_index
// that is unoccupied.
// If the column is full, return -1.
//
// Example:
//
//   0 1 2 3 4 5 6 7
// 0 _ _ _ _ _ _ B _
// 1 _ _ _ _ A _ B _
// 2 _ _ A B B _ A _
// 
// LowestOpenSpace(game, 0) =>  2
// LowestOpenSpace(game, 4) =>  0
// LowestOpenSpace(game, 3) =>  1
// LowestOpenSpace(game, 6) => -1
int LowestOpenSpace(const GameState *game, const int col_idx)
{
    // Start at the (visual) bottom of the board and walk up the 
    // specified column until we hit an empty cell.
    for (int i = game->board_height -1; i >= 0; i--) {
        printf("Checking cell at row %d, col %d\n", i, col_idx);
        printf("Cell value: %d\n", game->board[i][col_idx]);
        if (game->board[i][col_idx] == EMPTY) {
            printf("Lowest open space in column %d is at row %d\n", col_idx, i);
            return i;
        }
    }

    // If we reach the top of the column then it is full.
    return -1;
}


State EvaluateBoard(const GameState *game)
{
    // Get most recent move
    GameHistory *last_move = game->history;

    // Starting at the space that was just filled,
    // Walk in all directions to check for winner
    // If a winner is found, return the winner
    // If no winner is found, return TAKING_TURN_A or TAKING_TURN_B
    // If the board is full, return DRAW

    // Check for horizontal win
    int horizontal_count = 0;
    for (int i = 0; i < game->board_width; i++) {
        if (game->board[last_move->row][i] == last_move->team_placed) {
            horizontal_count += 1;
        } else {
            horizontal_count = 0;
        }

        if (horizontal_count >= 4) {
            return last_move->team_placed == PLAYER_A_VALUE ? WINNER_A : WINNER_B;
        }
    }

    // Check for vertical win
    int vertical_count = 0;
    for (int i = 0; i < game->board_height; i++) {
        if (game->board[i][last_move->col] == last_move->team_placed) {
            vertical_count += 1;
        } else {
            vertical_count = 0;
        }

        if (vertical_count >= 4) {
            return last_move->team_placed == PLAYER_A_VALUE ? WINNER_A : WINNER_B;
        }
    }

    // Check for diagonal win
    // TODO

    // Check for draw, starting at the (visual) top of the board
    int empty_count = 0;
    for (int i = 0; i < game->board_height; i++) {
        for (int j = 0; j < game->board_width; j++) {
            if (game->board[i][j] == EMPTY) {
                empty_count += 1;
            }
        }
    }

    if (empty_count == 0) {
        return DRAW;
    } else {
        return last_move->team_placed == PLAYER_A_VALUE ? TAKING_TURN_B : TAKING_TURN_A;
    }
}

