#ifndef __LOG_H
#define __LOG_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <string.h>

void rust_log(const void* data, size_t len);
#define rust_log_str(S) rust_log(S, strlen(S))
void rust_log_hex(const void* data, size_t len);

#ifdef __cplusplus
}
#endif

#endif
