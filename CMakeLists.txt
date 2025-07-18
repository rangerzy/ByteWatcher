cmake_minimum_required(VERSION 3.16)

project(ByteDealer VERSION 0.1 LANGUAGES CXX)

set(PROJECT_NAME "appByteDealer")
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Quick )

qt_standard_project_setup(REQUIRES 6.8)

qt_add_executable(${PROJECT_NAME}
    Main.cpp
)

qt_add_qml_module(${PROJECT_NAME}
    URI ByteDealer
    VERSION 1.0
    RESOURCES qml.qrc
    SOURCES connetinfo.h connetinfo.cpp
)
target_include_directories(${PROJECT_NAME} PRIVATE
    serial
)
target_link_libraries(${PROJECT_NAME} PRIVATE
     ${CMAKE_CURRENT_SOURCE_DIR}/serial/serial.lib
)


# Qt for iOS sets MACOSX_BUNDLE_GUI_IDENTIFIER automatically since Qt 6.1.
# If you are developing for iOS or macOS you should consider setting an
# explicit, fixed bundle identifier manually though.
set_target_properties(appByteDealer PROPERTIES
#    MACOSX_BUNDLE_GUI_IDENTIFIER com.example.appByteDealer
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_link_libraries(appByteDealer
    PRIVATE Qt6::Quick
)

include(GNUInstallDirs)
install(TARGETS appByteDealer
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
