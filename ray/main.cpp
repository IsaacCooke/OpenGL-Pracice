#include <cmath>
#include <iostream>
#include <ostream>
#include <raylib.h>

Vector2 normalise(Vector2 inputVector){
  float magnitude = sqrt(inputVector.x + inputVector.y);
  inputVector.x /= magnitude;
  inputVector.y /= magnitude;
  std::cout << inputVector.y << std::endl;
  return inputVector;
}

int main(void){
  const int screenWidth = 800;
  const int screenHeight = 450;

  InitWindow(screenWidth, screenHeight, "Raylib Test");

  Vector2 playerPosition = { (float)screenWidth / 2, (float)screenHeight / 2 };

  SetTargetFPS(60);

  while (!WindowShouldClose()) {
    if(IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) playerPosition.x += 2.0f;
    if(IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) playerPosition.x -= 2.0f;
    if(IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) playerPosition.y -= 2.0f;
    if(IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) playerPosition.y += 2.0f;
    normalise(playerPosition);

    BeginDrawing();

    ClearBackground(RAYWHITE);

    DrawCircleV(playerPosition, 50.0f, RED);

    EndDrawing();
  }

  CloseWindow();

  return 0;
}
