# Variables
BUILD_DIR = dist
CMAKE_FLAGS =
ifdef WASM
EMSDK_PATH = $(shell pwd)/.emsdk
endif

.PHONY: all
all: debug

# .PHONE: setup_emsdk
# setup_emsdk:
# 	git clone 

.PHONY: debug
debug: CMAKE_FLAGS += -DCMAKE_BUILD_TYPE=Debug
debug: configure build_cpp

.PHONY: release
release: CMAKE_FLAGS += -DCMAKE_BUILD_TYPE=Release
release: configure build_cpp

.PHONY: configure
configure:
	@mkdir -p cpp/$(BUILD_DIR)
ifdef EMSDK_PATH
	@source $(EMSDK_PATH)/emsdk_env.sh && cd cpp/$(BUILD_DIR) && emcmake cmake $(CMAKE_FLAGS) ..
else
	cd cpp/$(BUILD_DIR) && cmake $(CMAKE_FLAGS) ..
endif

.PHONY: build_cpp
build_cpp:
	cd cpp/$(BUILD_DIR) && $(MAKE)

.PHONY: build_rust_wheel
build_rust_wheel:
ifdef EMSDK_PATH
	@source $(EMSDK_PATH)/emsdk_env.sh && cd rust && maturin build --target=wasm32-unknown-emscripten -i python3.12
else
	cd rust && maturin build -i python3.12
endif

.PHONY: clean
clean:
	rm -rf cpp/$(BUILD_DIR)
	cd rust && cargo clean