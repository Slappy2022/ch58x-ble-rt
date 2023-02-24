#ifndef __RUST_BLE_H
#define __RUST_BLE_H

#include <stddef.h>

#ifndef uint8_t
typedef unsigned char uint8_t;
#endif

#ifndef uint16_t
typedef unsigned short uint16_t;
#endif

#ifndef uint32_t
typedef unsigned int uint32_t;
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  uint8_t len;
  const uint8_t *uuid;
} gattAttrType_t;

typedef struct gattAttribute_t {
  gattAttrType_t type;
  uint8_t permissions;
  uint16_t handle;
  uint8_t *pValue;
} gattAttribute_t;

void BleInit(uint8_t *mac, uint8_t tx_power);
uint8_t GAPBondMgr_PasscodeRsp(uint16_t connectionHandle, uint8_t status,
                               uint32_t passcode);
uint8_t GAPRole_PeripheralInit();
uint8_t GAPRole_SetParameter(uint16_t param, uint16_t len, void *pValue);
uint8_t GATTServApp_AddService(uint32_t services);
uint8_t GGS_AddService(uint32_t services);
void TMOS_SystemProcess(void);
uint8_t TMOS_TimerInit(void *fnGetClock);
void disable_broadcast();
uint32_t get_mac_address(uint8_t *buf);
void start_device();
uint8_t tmos_msg_deallocate(uint8_t *msg_ptr);
uint8_t tmos_msg_receive(uint8_t taskID, uint8_t *msg_ptr);

#ifdef __cplusplus
}
#endif

#endif // #define __RUST_BLE_H
