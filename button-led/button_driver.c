#include <sancus/reactive.h>

#include <stdio.h>

SM_OUTPUT(button_driver, button_pressed);

SM_INPUT(button_driver, trigger_button_press, data, len) {
  puts("[button_driver] Button has been pressed, sending output");
  uint16_t dummy = 0;
  button_pressed(&dummy, 2);
}
