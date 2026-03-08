#include <catch2/catch_session.hpp>

#include <lvgl.h>

int main( int argc, char* argv[] ) {
  lv_init();

  lv_test_display_create(800, 480);

  int result = Catch::Session().run( argc, argv );

  lv_deinit();

  return result;
}