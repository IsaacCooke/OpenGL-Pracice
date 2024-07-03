#include <raylib.h>

class Player {
public:
  Vector2 movement;
  Vector2 position;
  float speed = 1.1;

private:
  void init(int& screenHeight, int& screenWidth);
  void update();
  void draw();
};
