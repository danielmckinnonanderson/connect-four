#include <stdio.h>
#include <stdlib.h>
#include "raylib.h"

#define WINDOW_HEIGHT 720
#define WINDOW_WIDTH 720
#define WINDOW_TITLE "Wonderful World"
#define TARGET_FPS 60

#define BOARD_HEIGHT 8
#define BOARD_WIDTH 8

#define BOARD_CELL_SIZE WINDOW_HEIGHT / BOARD_HEIGHT

#define COLOR_A RED
#define COLOR_A_BG MAROON

#define COLOR_B BLUE
#define COLOR_B_BG DARKBLUE

typedef enum BoardOccupant {
    EMPTY = 0,
    PLAYER_A = 1,
    PLAYER_B = 2,
} BoardOccupant;

typedef enum State {
    INIT = 0,
    TAKING_TURN_A = 1,
    TAKING_TURN_B = 2,
    WINNER_A = 3,
    WINNER_B = 4,
    DRAW = 5,
} State;

typedef struct GameState {
    State state;
    int board[BOARD_HEIGHT][BOARD_WIDTH];
    int board_height;
    int board_width;
} GameState;


void DrawGame(const GameState *game, const Vector2 *mouse_pos);
int LowestOpenSpace(const GameState *game, const int col);

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
        Vector2 mouse_pos = GetMousePosition();
        switch (game.state)
        {
            case INIT:
                // Initialize the game state
                game.state = TAKING_TURN_A;
                break;
            case TAKING_TURN_A:
                if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
                    printf("Mouse position: %f, %f\n", mouse_pos.x, mouse_pos.y);
                    int col = (int) (mouse_pos.x / ((float)BOARD_CELL_SIZE));
                    printf("Col: %d\n", col);

                    int lowest_open_row = LowestOpenSpace(&game, col);
                    if (lowest_open_row == -1) {
                        // Column is full
                        printf("Column %d is full\n", col);
                        break;
                    }

                    // Column has an open space, place the piece
                    game.board[lowest_open_row][col] = PLAYER_A;
                    game.state = TAKING_TURN_B;
                }
                break;
            case TAKING_TURN_B:
                if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
                    printf("Mouse position: %f, %f\n", mouse_pos.x, mouse_pos.y);
                    int col = (int) (mouse_pos.x / ((float)BOARD_CELL_SIZE));
                    printf("Col: %d\n", col);

                    int lowest_open_row = LowestOpenSpace(&game, col);
                    if (lowest_open_row == -1) {
                        // Column is full
                        printf("Column %d is full\n", col);
                        break;
                    }

                    // Column has an open space, place the piece
                    game.board[lowest_open_row][col] = PLAYER_B;
                    game.state = TAKING_TURN_A;
                }
                break;
            case WINNER_A:
                break;
            case WINNER_B:
                break;
            case DRAW:
                break;
            default:
                printf("Invalid game state: %d\n", game.state);
                exit(1);
                break;
        }

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


void DrawGame(const GameState *game, const Vector2 *mouse_pos)
{
    ClearBackground(BLACK);

    // If mouse position is within column, highlight entire column
    int col = (int) (mouse_pos->x / ((float)BOARD_CELL_SIZE));
    
    if (col >= 0 && col < game->board_width) {
        Color team_bg_highlight = game->state == TAKING_TURN_A ? MAROON : DARKBLUE;
        DrawRectangle(
            col * BOARD_CELL_SIZE, 
            0, 
            BOARD_CELL_SIZE, 
            WINDOW_HEIGHT, 
            team_bg_highlight
        );
    }

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

