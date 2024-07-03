#include <cmath>
#include <raylib.h>
#include "gameplay/player.h"

Vector2 normalise(Vector2 inputVector) {
    float magnitude = sqrt(inputVector.x * inputVector.x + inputVector.y * inputVector.y);
    if (magnitude != 0) {  // Prevent division by zero
        inputVector.x /= magnitude;
        inputVector.y /= magnitude;
    }
    return inputVector;
}

int main(void) {
    const int screenWidth = 800;
    const int screenHeight = 450;

    InitWindow(screenWidth, screenHeight, "Raylib Test");

  Player player;
  player.init(*screenHeight, &screenWidth);

    Vector2 playerPosition = { (float)screenWidth / 2, (float)screenHeight / 2 };
    Vector2 playerMovement = {0.0, 0.0};
    float playerSpeed = 1.1;

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        playerMovement = {0.0, 0.0};
        if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) playerMovement.x += 2.0f;
        if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) playerMovement.x -= 2.0f;
        if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) playerMovement.y -= 2.0f;
        if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) playerMovement.y += 2.0f;

        playerMovement = normalise(playerMovement);

        playerPosition.x += playerMovement.x * playerSpeed;
        playerPosition.y += playerMovement.y * playerSpeed;

        BeginDrawing();

        ClearBackground(RAYWHITE);

        DrawCircleV(playerPosition, 50.0f, RED);

        EndDrawing();
    }

    CloseWindow();

    return 0;
}
