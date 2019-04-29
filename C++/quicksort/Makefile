CXX=g++ -m64
CXXFLAGS=-I -Iobjs/ -O3 -Wall -fopenmp -D_GLIBCXX_PARALLEL

all: quicksort
#quicksort-parallel quicksort-pool

quicksort: quicksort.cpp
	$(CXX) $(CXXFLAGS) $< -o $@

#quicksort-parallel: quicksort-parallel.c
#	gcc -g -Wall $< -o $@

#quicksort-pool: quicksort-pool.c
#	gcc -g -Wall $< -o $@

clean:
	rm -f quicksort
#quicksort-parallel quicksort-pool

                                                                                                      