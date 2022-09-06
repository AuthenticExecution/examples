#include <sancus/reactive.h>

#include <stdio.h>

SM_OUTPUT({name}, send_pong);

SM_INPUT({name}, recv_ping, data, len) {
  puts("[{name}] received ping");
  send_pong(data, len);
}
