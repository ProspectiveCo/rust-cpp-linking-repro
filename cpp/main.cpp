#include "helloworld.h"

int main() {
  auto internal = AllocateAndPrint();
  delete internal;
  return 0;
}