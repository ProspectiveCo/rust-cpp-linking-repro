#include <cstdint>
#include <iostream>

class Internal {
public:
  Internal() { std::cout << "Hello!" << '\n'; }
  ~Internal() { std::cout << "Goodbye!" << '\n'; }

  std::uint32_t data[16];
  std::uint32_t moreData[16];
};

Internal *AllocateAndPrint();