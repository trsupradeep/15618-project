CXX=g++ -m64
CXXFLAGS=-I -Iobjs/ -O3 -Wall
OMP=-fopenmp -DOMP

APP_NAME=mandelbrot
OBJDIR=objs
LDFLAGS= -lm

all: mandel

.PHONY: dirs clean 

dirs:
		/bin/mkdir -p $(OBJDIR)/

clean:
		/bin/rm -rf $(OBJDIR) *.ppm *~ $(APP_NAME)


OBJS=$(OBJDIR)/main.o

$(OBJDIR)/%.o: %.cpp
		$(CXX) $< $(CXXFLAGS) -c -o $@


$(OBJDIR)/main.o: CycleTimer.h
 

mandel:dirs $(OBJS) 
		$(CXX) $(OMP) $(CXXFLAGS) -o $@ $(OBJS) $(LDFLAGS) 		


                                                                                                      