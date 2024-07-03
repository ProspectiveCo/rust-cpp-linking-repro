#include "helloworld.h"
#include <iostream>

static constexpr size_t internalSize = sizeof(Internal);

Internal *AllocateAndPrint() {
  Internal *a = static_cast<Internal *>(::operator new(256));
  new (a) Internal();
  return a;
}