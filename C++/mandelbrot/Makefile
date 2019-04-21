CXX=g++ -m64
CXXFLAGS=-I./common -Iobjs/ -O3 -Wall
OMP=-fopenmp -DOMP

APP_NAME=mandelbrot
OBJDIR=objs
COMMONDIR=./common
LDFLAGS= -lm

PPM_CXX=$(COMMONDIR)/ppm.cpp
PPM_OBJ=$(addprefix $(OBJDIR)/, $(subst $(COMMONDIR)/,, $(PPM_CXX:.cpp=.o)))


all: mandel

.PHONY: dirs clean 

dirs:
		/bin/mkdir -p $(OBJDIR)/

clean:
		/bin/rm -rf $(OBJDIR) *.ppm *~ $(APP_NAME)


OBJS=$(OBJDIR)/main.o $(OBJDIR)/mandelbrot.o $(PPM_OBJ)

$(OBJDIR)/%.o: %.cpp
		$(CXX) $< $(CXXFLAGS) -c -o $@

$(OBJDIR)/%.o: $(COMMONDIR)/%.cpp
	$(CXX) $< $(CXXFLAGS) -c -o $@

$(OBJDIR)/main.o: $(COMMONDIR)/CycleTimer.h
 

mandel:dirs $(OBJS) 
		$(CXX) $(OMP) $(CXXFLAGS) -o $@ $(OBJS)  $(LDFLAGS) 		


                                                                                                      