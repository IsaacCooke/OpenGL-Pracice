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
