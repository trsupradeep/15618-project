#include <algorithm>
#include <getopt.h>
#include <omp.h>
#include <stdio.h>

#include "CycleTimer.h"

#define VIEWCNT 7
#define IMAGE_HEIGHT 2048
#define IMAGE_WIDTH 2048
#define NUM_ITER 256
#define NUM_THREADS 2
#define NUM_RUNS 1

// Core computation of Mandelbrot set membershop
// Iterate complex number c to determine whether it diverges
static inline int mandel(float c_re, float c_im, int count) {
  float z_re = c_re, z_im = c_im;
  int i;
  for (i = 0; i < count; ++i) {
    if (z_re * z_re + z_im * z_im > 4.f)
      break;

    float new_re = z_re * z_re - z_im * z_im;
    float new_im = 2.f * z_re * z_im;
    z_re = c_re + new_re;
    z_im = c_im + new_im;
  }
  return i;
}

void usage(const char *progname) {
  printf("Usage: %s [options]\n", progname);
  printf("Program Options:\n");
  printf("  -t  --threads <N>       Use N threads\n");
  printf("  -v  --view <INT>        Use specified view settings (0-6)\n");
  printf("  -f  --field x0:y0:x1:y1 Specify set boundaries\n");
  printf("  -c  --code configuration Specify 0 for all, 1 for parallel only, 2 "
         "for serial only\n");
  printf("  -?  --help              This message\n");
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

// Function to shift and scale complex imputs
void scaleAndShift(float &x0, float &x1, float &y0, float &y1, float scale,
                   float shiftX, float shiftY) {
  x0 *= scale;
  x1 *= scale;
  y0 *= scale;
  y1 *= scale;
  x0 += shiftX;
  x1 += shiftX;
  y0 += shiftY;
  y1 += shiftY;
}

// Serial version of Mandelbrot
void mandelbrotSerial(float x0, float y0, float x1, float y1, int width,
                      int height, int maxIterations, int output[]) {
  float dx = (x1 - x0) / width;
  float dy = (y1 - y0) / height;

  for (int j = 0; j < height; j++) {
    for (int i = 0; i < width; ++i) {
      float x = x0 + i * dx;
      float y = y0 + j * dy;
      int index = (j * width + i);
      output[index] = mandel(x, y, maxIterations);
    }
  }
}

//Parallel over pixels of the image
void mandelbrot_pixel_parallel(int numThreads, float x0, float y0, float x1,
                             float y1, int width, int height, int
                             maxIterations, int output[]) {
  float dx = (x1 - x0) / width;
  float dy = (y1 - y0) / height;

  int i, j;
  omp_set_num_threads(numThreads);
    for (j = 0; j < height; j++) {
      #pragma omp parallel for private(i) schedule(static)
      for (i = 0; i < width; ++i) {
        float x = x0 + i * dx;
        float y = y0 + j * dy;
        int index = (j * width + i);
        //printf("index = %d, thread = %d\n",index, omp_get_thread_num());
        output[index] = mandel(x, y, maxIterations);
      }
  }
  }

// Parallel over each row of the image
void mandelbrot_row_parallel(int numThreads, float x0, float y0, float x1,
                               float y1, int width, int height,
                               int maxIterations, int output[]) {
  float dx = (x1 - x0) / width;
  float dy = (y1 - y0) / height;

  int i, j;
  omp_set_num_threads(numThreads);
#pragma omp parallel for private(i) schedule(static)
    for (j = 0; j < height; j++) {
      for (i = 0; i < width; ++i) {
        float x = x0 + i * dx;
        float y = y0 + j * dy;
        int index = (j * width + i);
        //printf("index = %d, thread = %d\n",index, omp_get_thread_num());
        output[index] = mandel(x, y, maxIterations);
      }
  }
}

void do_runs(float x0, float x1, float y0, float y1, int width, int height,
             int maxIterations, int numThreads, int code_config) {
  int *output_serial = new int[width * height];
  int *output_parallel = new int[width * height];
  int *output_parallel_row = new int[width * height];
  memset(output_serial, 0, width * height * sizeof(int));
  double minSerial = 1e30;
  memset(output_parallel, 0, width * height * sizeof(int));
  memset(output_parallel_row, 0, width * height * sizeof(int));
  double minThread = 1e30;
  // Runs serial
  if ((code_config == 0) || (code_config == 1)) {
    for (int i = 0; i < NUM_RUNS; ++i) {
      double startTime = CycleTimer::currentSeconds();
      mandelbrotSerial(x0, y0, x1, y1, width, height, maxIterations,
                       output_serial);
      double endTime = CycleTimer::currentSeconds();
      minSerial = std::min(minSerial, endTime - startTime);
    }
    printf("[mandelbrot serial]:\t\t[%.3f] ms\n", minSerial * 1000);
  }
  // Runs parallel versions
  if ((code_config % 2) == 0) {

    printf("Running Parallel over pixels\n");
    for (int i = 0; i < NUM_RUNS; ++i) {
      double startTime = CycleTimer::currentSeconds();
      mandelbrot_pixel_parallel(numThreads, x0, y0, x1, y1, width, height,
                                maxIterations, output_parallel);
      double endTime = CycleTimer::currentSeconds();
      minThread = std::min(minThread, endTime - startTime);
    }
    printf("[mandelbrot thread - over pixels]:\t\t[%.3f] ms\n", minThread * 1000);
    if (!verifyResult(output_serial, output_parallel, width, height)) {
      printf("ERROR : Output from threads does not match serial output\n");
      delete[] output_serial;
      delete[] output_parallel;
    }
    // compute speedup
    printf("++++\t\t\t\t(%.2fx speedup from %d threads parallel over pixels)\n",
           minSerial / minThread, numThreads);

    printf("Running Parallel over rows\n");
    for (int i = 0; i < NUM_RUNS; ++i) {
      double startTime = CycleTimer::currentSeconds();
      mandelbrot_row_parallel(numThreads, x0, y0, x1, y1, width, height,
                                maxIterations, output_parallel_row);
      double endTime = CycleTimer::currentSeconds();
      minThread = std::min(minThread, endTime - startTime);
    }
    printf("[mandelbrot thread - over rows]:\t\t[%.3f] ms\n", minThread * 1000);
    if (!verifyResult(output_serial, output_parallel_row, width, height)) {
      printf("ERROR : Output from threads does not match serial output\n");
      delete[] output_serial;
      delete[] output_parallel_row;
    }
    // compute speedup
    printf("++++\t\t\t\t(%.2fx speedup from %d threads parallel over rows)\n",
           minSerial / minThread, numThreads);       
  }

  delete[] output_serial;
  delete[] output_parallel;
  delete[] output_parallel_row;
}

int main(int argc, char **argv) {

  const int width = IMAGE_WIDTH;
  const int height = IMAGE_HEIGHT;
  const int maxIterations = NUM_ITER;
  int numThreads = NUM_THREADS;

  float x0 = -2.167;
  float x1 = 1.167;
  float y0 = -1;
  float y1 = 1;

  // Support VIEWCNT views
  float scaleValues[VIEWCNT] = {0.01f, 1.0f,  0.015f, 0.02f,
                                0.02f, 0.02f, 0.002f};
  float shiftXs[VIEWCNT] = {0.0f, 0.0f, -0.98f, 0.35f, 0.0f, -1.5f, -1.4f};
  float shiftYs[VIEWCNT] = {0.0f, 0.0f, 0.30f, 0.05f, 0.73f, 0.0f, 0.0f};

  // parse commandline options ////////////////////////////////////////////
  int opt;
  static struct option long_options[] = {
      {"threads", 1, 0, 't'},     {"view", 1, 0, 'v'}, {"field", 1, 0, 'f'},
      {"code_config", 1, 0, 'c'}, {"help", 0, 0, '?'}, {0, 0, 0, 0}};

  int viewIndex = 1;
  int code_config = 0;
  while ((opt = getopt_long(argc, argv, "t:v:f:c:?", long_options, NULL)) !=
         EOF) {

    switch (opt) {
    case 't': {
      numThreads = atoi(optarg);
      break;
    }
    case 'v': {
      viewIndex = atoi(optarg);
      // change view settings
      if (viewIndex < 0 || viewIndex >= VIEWCNT) {
        fprintf(stderr, "Invalid view index %d\n", viewIndex);
        return 1;
      }
      break;
    }
    case 'f': {
      if (sscanf(optarg, "%f:%f:%f:%f", &x0, &y0, &x1, &y1) != 4) {
        fprintf(stderr, "Couldn't extract field from '%s'\n", optarg);
        exit(1);
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
    case '?':
    default:
      usage(argv[0]);
      return 1;
    }
  }
  // end parsing of commandline options
  float scaleValue = scaleValues[viewIndex];
  float shiftX = shiftXs[viewIndex];
  float shiftY = shiftYs[viewIndex];
  scaleAndShift(x0, x1, y0, y1, scaleValue, shiftX, shiftY);

  do_runs(x0, x1, y0, y1, width, height, maxIterations, numThreads,
          code_config);
  return 0;
}