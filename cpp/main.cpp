#include "helloworld.h"

int myfn() {
  auto internal = AllocateAndPrint();
  delete internal;
  return 0;
}