#include <sancus/reactive.h>

#include <stdio.h>

SM_INPUT({name}, receive, data, len) {
  puts("[{name}] received data");
}
