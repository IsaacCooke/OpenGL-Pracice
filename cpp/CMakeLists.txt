# CMake entry point
cmake_minimum_required (VERSION 3.0)
project (Ogl)

# Find OpenGL library
find_package(OpenGL REQUIRED) 

# Include directories
include_directories(
  external/glfw-3.1.2/include/
  external/glm-0.9.7.1/
  external/glew-1.13.0/include/
)

# Libraries
set(ALL_LIBS
  ${OPENGL_LIBRARY}
  glfw
)

# Common files
file(GLOB_RECURSE COMMON "common/*.cpp")

# Add main executable
add_executable(main main.cpp ${COMMON})

# Link libraries
target_link_libraries(main 
  ${ALL_LIBS}
  GLEW
)
