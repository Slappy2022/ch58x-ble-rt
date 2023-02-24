#include "log.h"

static const char HEX[] = "0123456789abcdef";

void rust_log_hex(const void *data, size_t len) {
  const char *data_bytes = (const char *)data;
  char buf[130];
  buf[0] = '0';
  buf[1] = 'x';
  len = len < 64 ? len : 64;
  for (size_t i = 0; i < len; i++) {
    buf[2 * i + 2] = HEX[data_bytes[i] >> 4];
    buf[2 * i + 3] = HEX[data_bytes[i] & 0x0f];
  }
  rust_log(buf, 2 * len + 2);
}
