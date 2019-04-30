CXX=g++ -m64
CXXFLAGS=-I -Iobjs/ -O3 -Wall -fopenmp

LDFLAGS= -lm
OBJDIR=objs
all: quicksort
#quicksort-parallel quicksort-pool

$(Header): CycleTimer.h

quicksort: main.cpp $(Header)
	$(CXX) $(CXXFLAGS) $< -o $@ $(OBJS) $(LDFLAGS)

clean:
	rm -f quicksort
#quicksort-parallel quicksort-pool

                                                                                                      