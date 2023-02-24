#ifndef __RUST_SERVICE_H
#define __RUST_SERVICE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct service_ptr_t {
  void *ptr;
} service_ptr_t;

typedef struct characteristic_info_t {
  uint8_t uuid[16];
  uint8_t is_short;
  uint8_t properties;
  uint8_t permissions;
} characteristic_info_t;

typedef struct service_info_t {
  uint8_t uuid[16];
  uint8_t is_short;
  size_t characteristicsLen;
  const characteristic_info_t *characteristics;
} service_info_t;

uint8_t register_service(service_ptr_t *ptr, const service_info_t *info);

#ifdef __cplusplus
}
#endif

#endif // #define __RUST_SERVICE_H
