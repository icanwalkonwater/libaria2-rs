# This CMake file is not meant to be used for compilation, it is here to help CLion find the generated headers
# for better autocompletion.

cmake_minimum_required(VERSION 3.17)

PROJECT(libaria2-rs)
add_library(STATIC src/aria2_bridge.cpp)
# _clion_include/ contains symlinks to reflect the hierarchy inside target/cxxbridge during compilation (with the headers)
include_directories(target/cxxbridge _clion_include)
