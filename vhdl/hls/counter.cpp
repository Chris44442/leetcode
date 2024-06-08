#include "HLS/hls.h"

#define BUFFER_SIZE 128

hls_always_run_component
component void mycomp_xyz (
  hls_avalon_agent_memory_argument(BUFFER_SIZE*sizeof(int)) int* y)
{
  // y[0] = 5;
  // y[1] = 17;
  // int tmp = y[2];
  // y[3] = tmp;
  // y[4] = 7;
  // int tmp2 = y[4];
  // y[5] = tmp2;
  // return y[1];
}

// int main(){return 0;}


// #include "HLS/hls.h"
// #include <stdio.h>

// using namespace ihc;

// component unsigned int count() {
  // static unsigned int cnt = 0;
  // return cnt++;
// }

int main() {
  constexpr int SIZE = 400;
  int y[BUFFER_SIZE];
  for(unsigned int i=0; i<SIZE; ++i) {
    mycomp_xyz(y);


    // cout << "Pointer value: " << hex << *y[i] << endl;

    printf("%p : %u \n",(void*)&y[i], y[i]);
  }
// printf("%p\n",(void*)&a);
  // ihc_hls_component_run_all(count);

  // bool pass = true;
  // for(unsigned int i=0; i < SIZE; ++i) {

    // printf("Expected %u, Actual %u\n", i, result[i]);
    // if (result[i] != i) {
      // printf("ERROR: Expected %u, found %u\n", i, result[i]);
      // pass = false;
    // }
  // }

  // if (pass) {
    // printf("PASSED\n");
  // }
  // else {
    // printf("FAILED\n");
  // }

  return 0;

}
