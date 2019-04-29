CXX=g++ -m64
CXXFLAGS=-I -Iobjs/ -O3 -Wall -fopenmp

APP_NAME=matrix_multiply
OBJDIR=objs
LDFLAGS= -lm

all: matmul

.PHONY: dirs clean 

dirs:
		/bin/mkdir -p $(OBJDIR)/

clean:
		/bin/rm -rf $(OBJDIR) *.ppm *~ $(APP_NAME)


OBJS=$(OBJDIR)/main.o

$(OBJDIR)/%.o: %.cpp
		$(CXX) $< $(CXXFLAGS) -c -o $@


$(OBJDIR)/main.o: CycleTimer.h
 

matmul:dirs $(OBJS) 
		$(CXX) $(CXXFLAGS) -o $@ $(OBJS) $(LDFLAGS) 		


                                                                                                      