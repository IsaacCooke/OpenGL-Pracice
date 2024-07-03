#include "player.h"
#include <raylib.h>

void Player::init(int& screenHeight, int& screenWidth){
  this->movement = {0, 0};
  this->position = {(float)screenWidth / 2, (float)screenHeight / 2};
}

void Player::update(){
  this->movement = {0.0, 0.0};
  if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) this->movement.x += 2.0f;
  if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) this->movement.x -= 2.0f;
  if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) this->movement.y -= 2.0f;
  if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) this->movement.y += 2.0f;

  this->position.x += this->movement.x;
  this->position.y += this->movement.y;
}

void Player::draw(){
  DrawCircleV(this->position, 50, GREEN);
}
