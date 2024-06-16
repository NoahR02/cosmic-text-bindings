cmake_minimum_required(VERSION 3.15)

project(cosmic-text VERSION 1.0 LANGUAGES C CXX)

add_library(cosmic-text STATIC IMPORTED)
set_target_properties(${PROJECT_NAME} PROPERTIES
        IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/lib/cosmic_text_c.lib"
        INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

if(WIN32)
    target_link_libraries(${PROJECT_NAME} INTERFACE kernel32.lib advapi32.lib kernel32.lib ntdll.lib userenv.lib ws2_32.lib kernel32.lib ws2_32.lib kernel32.lib msvcrt.lib)
elseif (UNIX)
    target_link_libraries(${PROJECT_NAME} INTERFACE pthread dl)
else ()
    message(FATAL_ERROR "Unsupported platform")
endif ()

if(BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()