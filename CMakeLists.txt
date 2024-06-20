cmake_minimum_required(VERSION 3.15)

project(cosmic-text VERSION 1.0 LANGUAGES CXX)

add_library(cosmic-text STATIC include/cosmic-text.hpp include/cosmic-text.cpp)

add_library(cosmic-text-lib STATIC IMPORTED)
set_target_properties(cosmic-text-lib PROPERTIES IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/lib/cosmic_text_c.lib")
if(WIN32)
    message(INFO "working")
    target_link_libraries(cosmic-text-lib PRIVATE INTERFACE ntdll.lib userenv.lib ws2_32.lib)
elseif (UNIX)
    target_link_libraries(cosmic-text-lib PRIVATE INTERFACE pthread dl)
else ()
    message(FATAL_ERROR "Unsupported platform")
endif ()

target_link_libraries(${PROJECT_NAME} PRIVATE INTERFACE cosmic-text-lib)
target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

if(BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()