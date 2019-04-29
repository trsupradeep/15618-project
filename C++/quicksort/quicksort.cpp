#include <getopt.h>
#include <omp.h>
#include <stdio.h>
#include <algorithm>
#include <parallel/algorithm>
#include <vector>

#define MAT_SIZE 1024
#define NUM_RUNS 1
#define NUM_THREADS 2

void usage(const char *progname) {
  printf("Usage: %s [options]\n", progname);
  printf("Program Options:\n");
  printf("  -t  --threads <N>       Use N threads\n");
  printf("  -r  --runs <INT>        Use specified number of runs\n");
  printf("  -s  --size <INT>        Use specified size of matrix\n");
  printf(
      "  -c  --code configuration Specify 0 for all, 1 for serial only, 2 "
      "for parallel only\n");
  printf("  -?  --help              This message\n");
}
void display(std::vector<int> &v) {
  std::vector<int>::size_type i;
  printf(">");
  for (i = 0; i < v.size(); i++) printf(" %d", v.at(i));
  printf("\n");
}
void do_runs(int numThreads, int code_config, int runs, int *array, int filesize) {
  std::vector<int> v;
  // Runs serial
  if ((code_config == 0) || (code_config == 1)) {
  }
  // Runs parallel versions
  if ((code_config % 2) == 0) {
    v.assign(array, array + filesize);
    __gnu_parallel::sort(v.begin(), v.end());
    display(v);
  }
}

int main(int argc, char **argv) {
  int runs = NUM_RUNS;
  int code_config = 0;
  int numThreads = NUM_THREADS;
  int filesize = 10;

  // parse commandline options ////////////////////////////////////////////
  int opt;
  static struct option long_options[] = {
      {"threads", 1, 0, 't'},     {"filesize", 1, 0, 'f'}, {"runs", 1, 0, 'r'},
      {"code_config", 1, 0, 'c'}, {"help", 0, 0, '?'},     {0, 0, 0, 0}};

  while ((opt = getopt_long(argc, argv, "t:f:r:c:?", long_options, NULL)) !=
         EOF) {
    switch (opt) {
      case 't': {
        numThreads = atoi(optarg);
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
      case 'f': {
        filesize = atoi(optarg);
        break;
      }
      case 'r': {
        runs = atoi(optarg);
        break;
      }
      case '?':
      default:
        usage(argv[0]);
        return 1;
    }
  }
  int *array = NULL;
  int length = 0;
  char filename[10];
  FILE *fh;
  int data;

  /* Initialize data. */
  printf("attempting to sort file: %d.txt\n", filesize);
  sprintf(filename, "%d%s", filesize, ".txt");
  fh = fopen(filename, "r");
  if (fh == NULL) {
    printf("error opening file\n");
    return 0;
  }

  while (fscanf(fh, "%d", &data) != EOF) {
    ++length;
    array = (int *)realloc(array, length * sizeof(int));
    array[length - 1] = data;
  }
  fclose(fh);

  do_runs(numThreads, code_config, runs, array, filesize);
  return 0;
}