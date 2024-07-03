cmake_minimum_required(VERSION 3.29)

project(HelloWorld)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 20)

set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -O0")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3")


if(EMSCRIPTEN)
  set(WASM_FLAGS " \
    --no-entry \
    -s ERROR_ON_UNDEFINED_SYMBOLS=0 \
    -s RELOCATABLE=1 \
    -s ALLOW_TABLE_GROWTH=1 \
    -s DYLINK_DEBUG=1 \
    -s LINKABLE=1 \
    -s EXPORT_ALL=1 \
    -s SIDE_MODULE=1 \
  ")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${WASM_FLAGS}")
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${WASM_FLAGS}")
  set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} ${WASM_FLAGS}")
  set(CMAKE_EXECUTABLE_SUFFIX ".wasm")
endif()

add_library(world src/helloworld.cpp)
target_sources(world PRIVATE src/helloworld.cpp)
target_include_directories(world PUBLIC include)
install(TARGETS world DESTINATION ${CMAKE_INSTALL_PREFIX})
install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_PREFIX}/include)

add_executable(${PROJECT_NAME} main.cpp)
target_link_libraries(${PROJECT_NAME} world)
if(EMSCRIPTEN)
  target_link_options(${PROJECT_NAME} PRIVATE -sENVIRONMENT="web")
  set_target_properties(${PROJECT_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "./web/")
endif()