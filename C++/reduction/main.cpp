#include <getopt.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <algorithm>
#include <cstring>
#include <time.h>

#include "CycleTimer.h"

#define SIZE 1 * 1000 * 1000
#define NUM_THREADS 16
#define NUM_RUNS 3
#define DEFAULT_DO_SQUARE 0
#define MAX_RANGE 10

#define RM(i, j, width) (i * width + j)

void usage(const char *progname) {
  printf("Usage: %s [options]\n", progname);
  printf("Program Options:\n");
  printf("  -t  --threads <N>       Use N threads\n");
  printf("  -n  --num <INT>        \n");
  printf(
      "  -c  --code configuration Specify 0 for all, 1 for parallel only, 2 "
      "for serial only\n");
  printf("  -d  --do_square     Perform squaring and reduction\n");
  printf("  -r  --runs     The number of runs to do\n");
  printf("  -?  --help              This message\n");
}

// Serial reduction
double reduction_serial(long int len, double *arr, bool do_square) {
  double sum = 0.0;
  if (do_square) {
    for (int i = 0; i < len; i++) {
      sum += arr[i] * arr[i];
    }
  } else {
    for (int i = 0; i < len; i++) {
      sum += arr[i];
    }
  }
  return sum;
}

// Parallel reduction
double reduction_par(long int len, double *arr, bool do_square) {
  double sum = 0.0;
  omp_set_num_threads(4);
  if (do_square) {
#pragma omp parallel for reduction(+ : sum)
    for (int i = 0; i < len; i++) {
      sum += arr[i] * arr[i];
    }
  } else {
#pragma omp parallel for reduction(+ : sum)
    for (int i = 0; i < len; i++) {
      sum += arr[i];
    }
  }
  return sum;
}

static void random_init(double *A, int len) {
  srand(time(NULL));
  for (int i = 0; i < len; i++) {
    A[i] = rand();
  }
  printf("Generated all random values! \n");
}

void do_runs(long int size, int do_square, int numThreads, int code_config,
             int numRuns) {
  double *arr_serial = new double[size];
  double *arr_parallel = new double[size];

  double minSerial = 1e30;
  double minThread = 1e30;

  random_init(arr_serial, size);
  memcpy(arr_parallel, arr_serial, size * sizeof(double));

  // Runs serial
  if ((code_config == 0) || (code_config == 1)) {
    for (int i = 0; i < numRuns; ++i) {
      double startTime = CycleTimer::currentSeconds();
      printf("Sum:%lf \n", reduction_serial(size, arr_serial, do_square));
      double endTime = CycleTimer::currentSeconds();
      minSerial = std::min(minSerial, endTime - startTime);
    }

    printf("[reduction-c++ serial]:\t\t[%.3f] ms\n", minSerial * 1000);
  }

  // Runs parallel versions
  if ((code_config == 0) || (code_config == 2)) {
    for (int i = 0; i < numRuns; ++i) {
      double startTime = CycleTimer::currentSeconds();
      printf("Sum:%lf \n", reduction_par(size, arr_parallel, do_square));
      double endTime = CycleTimer::currentSeconds();
      minThread = std::min(minThread, endTime - startTime);
    }
    printf("[reduction-c++ par]:\t\t[%.3f] ms\n", minThread * 1000);

    if (code_config == 0) {
      // compute speedup
      printf("++++\t\t\t\t(%.2fx speedup from %d threads)\n",
             minSerial / minThread, numThreads);
    }
  }

  delete[] arr_serial;
  delete[] arr_parallel;
}

int main(int argc, char **argv) {
  long int size = SIZE;
  int numThreads = NUM_THREADS;
  int do_square = DEFAULT_DO_SQUARE;
  int numRuns = NUM_RUNS;

  // parse commandline options ////////////////////////////////////////////
  int opt;
  static struct option long_options[] = {{"threads", 1, 0, 't'},
                                         {"num", 1, 0, 'n'},
                                         {"code_config", 1, 0, 'c'},
                                         {"do_square", 1, 0, 'p'},
                                         {"runs", 1, 0, 'r'},
                                         {"help", 0, 0, '?'},
                                         {0, 0, 0, 0}};
  int code_config = 0;
  while ((opt = getopt_long(argc, argv, "t:n:c:p:r:?", long_options, NULL)) !=
         EOF) {
    switch (opt) {
      case 't': {
        numThreads = atoi(optarg);
        break;
      }
      case 'n': {
        size = atoi(optarg);
        // change view settings
        if (size < 0) {
          fprintf(stderr, "Invalid number of elements %ld\n", size);
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
      case 'p': {
        do_square = atoi(optarg);
        if (do_square < 0 || do_square > 1) {
          fprintf(stderr, "Wrong power option %d\n", do_square);
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

  printf(
      "Configuration: size: %ld, do_square: %d, code_config: %d, threads: %d, "
      "runs: %d\n\n",
      size, do_square, code_config, numThreads, numRuns);

  do_runs(size, do_square, numThreads, code_config, numRuns);
  return 0;
}
