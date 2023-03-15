# Configure with [fxsdk build-fx] or [fxsdk build-cg], which provide the
# toolchain file and module path of the fxSDK

cmake_minimum_required(VERSION 3.15)
project(SimplexSolver)

include(GenerateG1A)
include(GenerateG3A)
include(Fxconv)
find_package(Gint 2.9 REQUIRED)

set(SOURCES
  src/main.c
  src/fraction.c
  src/m.c
  src/matrix.c
  src/simplex.c
  src/visualcell.c
  src/tableux.c
  src/input.c
  src/stages.c
)
# Shared assets, fx-9860G-only assets and fx-CG-50-only assets
set(ASSETS
  # ...
)
set(ASSETS_fx
  assets-fx/example.png
  # ...
)
set(ASSETS_cg
  assets-cg/example.png
  # ...
)

fxconv_declare_assets(${ASSETS} ${ASSETS_fx} ${ASSETS_cg} WITH_METADATA)

add_executable(simplexsolver ${SOURCES} ${ASSETS} ${ASSETS_${FXSDK_PLATFORM}})
target_compile_options(simplexsolver PRIVATE -Wall -Wextra -Os)
target_link_libraries(simplexsolver Gint::Gint)

if("${FXSDK_PLATFORM_LONG}" STREQUAL fx9860G)
  generate_g1a(TARGET simplexsolver OUTPUT "SimplexSolver.g1a"
    NAME "SimplexSolver" ICON assets-fx/icon.png)
elseif("${FXSDK_PLATFORM_LONG}" STREQUAL fxCG50)
  generate_g3a(TARGET simplexsolver OUTPUT "SimplexSolver.g3a"
    NAME "SimplexSolver" ICONS assets-cg/icon-uns.png assets-cg/icon-sel.png)
endif()
