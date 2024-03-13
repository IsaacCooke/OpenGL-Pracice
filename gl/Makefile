CXX = g++
CXXFLAGS = -g -Wall `pkg-config --cflags --libs glfw3 gl`

SOURCES = main.cpp

OBJECTS = $(SOURCES:.cpp=.o)

EXECUTABLE = program

all: $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CXX) $(CXXFLAGS) $(OBJECTS) -o $(EXECUTABLE)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(EXECUTABLE) $(OBJECTS)

run: $(EXECUTABLE)
	./$(EXECUTABLE)

# test: main.o
#     g++ -o Test  -L./lib -lglew32 -lglfw3 -lopengl32 -lglu32 -lgdi32 main.o
#
# test.o: main.cpp
#     g++ -I./include -c main.cpp
