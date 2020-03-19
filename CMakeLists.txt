cmake_minimum_required(VERSION 3.14)
project(codedetection_telegram)

set(CMAKE_CXX_STANDARD 17)

add_compile_definitions(HAVE_CURL)

# Download automatically, you can also just copy the conan.cmake file
if(NOT EXISTS "${CMAKE_BINARY_DIR}/conan.cmake")
    message(STATUS "Downloading conan.cmake from https://github.com/conan-io/cmake-conan")
    file(DOWNLOAD "https://github.com/conan-io/cmake-conan/raw/v0.15/conan.cmake"
            "${CMAKE_BINARY_DIR}/conan.cmake")
endif()

include(${CMAKE_BINARY_DIR}/conan.cmake)

conan_cmake_run(REQUIRES
        cli11/[>1.3.0]@bincrafters/stable
        spdlog/[>1.3.1]@bincrafters/stable
        tgbot/[>1.0]
        BASIC_SETUP
        BASIC
        BUILD missing)

add_executable(codedetection_telegram main.cpp)
target_link_libraries(codedetection_telegram ${CONAN_LIBS})

install(TARGETS codedetection_telegram RUNTIME DESTINATION "${CMAKE_INSTALL_PREFIX}/bin")
install(DIRECTORY DESTINATION "${CMAKE_INSTALL_PREFIX}/bin/logs")
