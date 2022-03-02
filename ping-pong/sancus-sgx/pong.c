#include <sancus/reactive.h>

#include <stdio.h>

SM_OUTPUT(pong, send_pong);

SM_INPUT(pong, recv_ping, data, len) {
  puts("[pong] received ping");
  send_pong(data, len);
}
