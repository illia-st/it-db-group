cmake_minimum_required(VERSION 3.23)
project(RestServer VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

find_package(oatpp 1.3.0 REQUIRED)
find_package(cppzmq CONFIG REQUIRED)

add_executable(RestServer main.cpp Controller.hpp Server.cpp Server.hpp)

target_link_libraries(RestServer 
PUBLIC oatpp::oatpp
PUBLIC cppzmq cppzmq-static)
set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
