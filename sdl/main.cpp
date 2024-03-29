#include <SDL2/SDL.h>
#include <SDL2/SDL_surface.h>
#include <SDL2/SDL_video.h>
#include <stdio.h>
#include <string>

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;

enum KeyPressSurfaces {
  KEY_PRESS_SURFACE_DEFAULT,
  KEY_PRESS_SURFACE_UP,
  KEY_PRESS_SURFACE_DOWN,
  KEY_PRESS_SURFACE_RIGHT,
  KEY_PRESS_SURFACE_TOTAL,
};

SDL_Window* gWindow = NULL;
SDL_Surface* gScreenSurface = NULL;
SDL_Surface* gHelloWorld = NULL;
SDL_Surface* currentSurface = NULL;

bool init();
bool loadMedia();
void close();
SDL_Surface* loadSurface(std::string path);

int main(int argc, char* args[]){
  if(!init()){
    printf("Failed to initialise");
  } else {
    if (!loadMedia()){
      printf("Failed to load media\n");
    } else {
      // Apply image
      SDL_BlitSurface(gHelloWorld, NULL, gScreenSurface, NULL);

      // Update surface
      SDL_UpdateWindowSurface(gWindow);

      SDL_Event e; 
      bool quit = false;
      while( quit == false ){
        while( SDL_PollEvent( &e ) ){
          if( e.type == SDL_QUIT ) quit = true;
        }
      }
    }
  }

  close();
  return 0;
}

bool init(){
  bool success = true;

  if(SDL_Init(SDL_INIT_VIDEO) < 0) {
    printf("SDL could not initialise! Error: %s\n", SDL_GetError());
    success = false;
  } else {
    gWindow = SDL_CreateWindow("Tutorial", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, SDL_WINDOW_SHOWN);

    if(gWindow == NULL) {
      printf("Window could not be created! Error: %s\n", SDL_GetError());
      success = false;
    } else {
      gScreenSurface = SDL_GetWindowSurface(gWindow);
    }
  }

  return success;
}

bool loadMedia()
{
  bool success = true;

  gHelloWorld = SDL_LoadBMP("hello.bmp");
  if(gHelloWorld == NULL){
    printf("Unable to load image %s! Error: %s\n", "hello.bmp", SDL_GetError());
    success = false;
  }

  return success;
}

SDL_Surface* loadSurface(std::string path){
  SDL_Surface* loadedSurface = SDL_LoadBMP(path.c_str());
  if(loadedSurface == NULL){
    printf("Unable to load image %s! Error: %s\n", path.c_str(), SDL_GetError());
  }

  return loadedSurface;
}

void close()
{
  SDL_FreeSurface(gHelloWorld);
  gHelloWorld = NULL;

  SDL_DestroyWindow(gWindow);
  gWindow = NULL;

  SDL_Quit();
}
