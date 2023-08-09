# CMakeList.txt : CMake project for Telekinetic, include source and define
# project specific logic here.

cmake_policy(SET CMP0111 NEW)
cmake_policy(SET CMP0074 NEW)
find_package(Boost 1.81.0)

# Add the directory containing the DLL
add_library(LeapDynamic SHARED IMPORTED)
if(WIN32)
set_target_properties(LeapDynamic PROPERTIES IMPORTED_LOCATION Leap.dll
IMPORTED_IMPLIB Leap.lib)
else()
SET(Boost_USE_STATIC_LIBS ON)
FIND_PACKAGE(Boost REQUIRED COMPONENTS log_setup log)
#set_property(TARGET LeapDynamic PROPERTY IMPORTED_LOCATION "libLeap.so")
endif()
# Add source to this project's executable.
add_executable(hand-tracker "src/client.cpp" "src/leap-mouse.cpp" "src/LeapListener.cpp")
set_target_properties(hand-tracker PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/hand_tracking)
target_link_directories(hand-tracker PRIVATE lib ${Boost_LIBRARY_DIRS})

if(WIN32)
target_link_libraries(hand-tracker ${Boost_LIBRARIES} LeapDynamic)
else()
target_link_libraries(hand-tracker PRIVATE ${Boost_LIBRARIES} ${CMAKE_SOURCE_DIR}/hand_tracking/lib/libLeap.so)
endif()

target_include_directories(hand-tracker PRIVATE ${Boost_INCLUDE_DIRS} include)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET hand-tracker PROPERTY CXX_STANDARD 17)
endif()
