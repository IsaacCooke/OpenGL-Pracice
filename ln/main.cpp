#include <stdlib.h>
#include <GL/glew.h>
#ifdef __APPLE__
#   include <GLUT/glut.h>
#else
#   include <GL/glut.h>
#endif

int main(int argc, char** argv)
{
  glutInit(&argc, argv);
  glutInitDisplayMode(GLUT_RGB | GLUT_DOUBLE);
  glutInitWindowSize(400, 300);
  glutCreateWindow("Hello World");
  glutDisplayFunc(&update_fade_factor);

  glewInit();
  if (!GLEW_VERSION_2_0){
    fprintf(stderr, "Faild to load resources");
    return 1;
  }

  glutMainLoop();
  return 0;
}
