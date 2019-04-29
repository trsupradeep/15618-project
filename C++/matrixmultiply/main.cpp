#include <getopt.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <algorithm>
#include <cstring>

#include "CycleTimer.h"

#define SIZE 1024
#define NUM_THREADS 16
#define NUM_RUNS 3
#define ZORDER_FLAG 0
#define MAX_RANGE 10

#define RM(i, j, width) (i * width + j)

void usage(const char *progname) {
  printf("Usage: %s [options]\n", progname);
  printf("Program Options:\n");
  printf("  -t  --threads <N>       Use N threads\n");
  printf(
      "  -s  --size <INT>        Enter the size of the side of matrix (it will "
      "be rounded to next power of 2)\n");
  printf(
      "  -c  --code configuration Specify 0 for all, 1 for parallel only, 2 "
      "for serial only\n");
  printf(
      "  -z  --znot     Test the Z-order layout style matrix multiplication\n");
  printf("  -r  --runs     The number of runs to do\n");
  printf("  -?  --help              This message\n");
}

void pretty_print_matrix(int *M, int size) {
  fprintf(stderr, "[ ");
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      fprintf(stderr, "%d ", M[RM(i, j, size)]);
    }
    fprintf(stderr, "\n");
  }
}

// Function to verify the functionality of parallel code
bool verifyResult(int *gold, int *result, int width, int height) {
  int i, j;
  int errLimit = 5;
  bool ok = true;

  for (i = 0; i < height; i++) {
    for (j = 0; j < width; j++) {
      if (gold[i * width + j] != result[i * width + j]) {
        printf("Mismatch : [%d][%d], Expected : %d, Actual : %d\n", i, j,
               gold[i * width + j], result[i * width + j]);
        ok = false;
        if (--errLimit <= 0) {
          printf(" ...\n");
          return ok;
        }
      }
    }
  }

  return ok;
}

// Serial version of Matrix multiplication
void matmul_serial(int size, int *A, int *B, int *C) {
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      for (int k = 0; k < size; k++) {
        C[RM(i, j, size)] += A[RM(i, k, size)] * B[RM(k, j, size)];
      }
    }
  }
}

// Serial version of Matrix multiplication
void matmul_serial2(int size, int *A, int *B, int *C) {
  for (int i = 0; i < size; i++) {
    int iOff = i * size;
    for (int j = 0; j < size; j++) {
      int c = 0;
      for (int k = 0; k < size; k++) {
        c += A[iOff + k] * B[RM(k, i, size)];
      }
      C[RM(i, j, size)] = c;
    }
  }
}

// Parallel version of Matrix multiplication for row
void matmul_par_row(int size, int *A, int *B, int *C, int numThreads) {
  omp_set_num_threads(numThreads);
#pragma omp parallel shared(C)
  {
#pragma omp for schedule(static)
    for (int i = 0; i < size; i++) {
      int iOff = i * size;
      for (int j = 0; j < size; j++) {
        int c = 0;
        for (int k = 0; k < size; k++) {
          c += A[iOff + k] * B[RM(k, i, size)];
        }
        C[RM(i, j, size)] = c;
      }
    }
  }
}

// Outer loop parallel
void matmul_par_row_outer(int size, int *A, int *B, int *C, int numThreads) {
  omp_set_num_threads(numThreads);

#pragma omp parallel for
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      for (int k = 0; k < size; k++) {
        C[RM(i, j, size)] += A[RM(i, k, size)] * B[RM(k, j, size)];
      }
    }
  }
}

static void random_init(int *A, int size) {
  for (int row = 0; row < size; row++) {
    for (int col = 0; col < size; col++) {
      int rval = rand();
      int sign = (rval % 2) ? 1 : -1;
      A[RM(row, col, size)] = sign * (rval % MAX_RANGE);
    }
  }
}

void do_runs(int size, int zorder, int numThreads, int code_config,
             int numRuns) {
  int *C_serial = new int[size * size];
  int *C_parallel = new int[size * size];

  memset(C_serial, 0, size * size * sizeof(int));
  double minSerial = 1e30;

  memset(C_parallel, 0, size * size * sizeof(int));
  double minThread = 1e30;

  int *A = new int[size * size];
  int *B = new int[size * size];

  random_init(A, size);
  memcpy(B, A, size * size * sizeof(int));

  // Runs serial
  if ((code_config == 0) || (code_config == 1)) {
    for (int i = 0; i < numRuns; ++i) {
      memset(C_serial, 0, size * size * sizeof(int));

      double startTime = CycleTimer::currentSeconds();
      matmul_serial(size, A, B, C_serial);
      double endTime = CycleTimer::currentSeconds();
      minSerial = std::min(minSerial, endTime - startTime);
    }

    printf("[matmul serial]:\t\t[%.3f] ms\n", minSerial * 1000);
  }

  // Runs parallel versions
  if ((code_config == 0) || (code_config == 2)) {
    printf("Running Parallel over Row\n");
    for (int i = 0; i < NUM_RUNS; ++i) {
      memset(C_parallel, 0, size * size * sizeof(int));

      double startTime = CycleTimer::currentSeconds();
      matmul_par_row(size, A, B, C_parallel, numThreads);
      double endTime = CycleTimer::currentSeconds();
      minThread = std::min(minThread, endTime - startTime);
    }
    printf("[matmul-c++ par_row]:\t\t[%.3f] ms\n", minThread * 1000);

    if (code_config == 0) {
      // compute speedup
      printf("++++\t\t\t\t(%.2fx speedup from %d threads)\n",
             minSerial / minThread, numThreads);
    }
  }

  // Runs parallel versions
  if ((code_config == 0) || (code_config == 2)) {
    printf("Running Parallel over outer loop\n");
    for (int i = 0; i < NUM_RUNS; ++i) {
      memset(C_parallel, 0, size * size * sizeof(int));

      double startTime = CycleTimer::currentSeconds();
      matmul_par_row(size, A, B, C_parallel, numThreads);
      double endTime = CycleTimer::currentSeconds();
      minThread = std::min(minThread, endTime - startTime);
    }
    printf("[matmul-c++ par_outer]:\t\t[%.3f] ms\n", minThread * 1000);

    if (code_config == 0) {
      // compute speedup
      printf("++++\t\t\t\t(%.2fx speedup from %d threads)\n",
             minSerial / minThread, numThreads);
    }
  }

  delete[] C_serial;
  delete[] C_parallel;
  delete[] A;
  delete[] B;
}

int main(int argc, char **argv) {
  int size = SIZE;
  int numThreads = NUM_THREADS;
  int zorder = ZORDER_FLAG;
  int numRuns = NUM_RUNS;

  // parse commandline options ////////////////////////////////////////////
  int opt;
  static struct option long_options[] = {{"threads", 1, 0, 't'},
                                         {"size", 1, 0, 's'},
                                         {"code_config", 1, 0, 'c'},
                                         {"zorder", 1, 0, 'z'},
                                         {"runs", 1, 0, 'r'},
                                         {"help", 0, 0, '?'},
                                         {0, 0, 0, 0}};
  int code_config = 0;
  while ((opt = getopt_long(argc, argv, "t:s:c:z:r:?", long_options, NULL)) !=
         EOF) {
    switch (opt) {
      case 't': {
        numThreads = atoi(optarg);
        break;
      }
      case 's': {
        size = atoi(optarg);
        // change view settings
        if (size < 0) {
          fprintf(stderr, "Invalid size %d\n", size);
          return 1;
        }
        break;
      }
      case 'c': {
        code_config = atoi(optarg);
        if (code_config < 0 || code_config > 2) {
          fprintf(stderr, "Invalid code configuration %d\n", code_config);
          return 1;
        }
        break;
      }
      case 'z': {
        zorder = atoi(optarg);
        if (zorder < 0 || zorder > 1) {
          fprintf(stderr, "Invalid z-order input %d\n", zorder);
          return 1;
        }
        break;
      }
      case 'r': {
        numRuns = atoi(optarg);
        if (numRuns < 0) {
          fprintf(stderr, "Invalid number of runs %d\n", numRuns);
          return 1;
        }
        break;
      }
      case '?':
      default:
        usage(argv[0]);
        return 1;
    }
  }
  // end parsing of commandline options

  do_runs(size, zorder, numThreads, code_config, numRuns);
  return 0;
}