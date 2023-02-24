#include "CH58xBLE_LIB.h"
#include "ISP583.h"
#include <string.h>

#ifndef BLE_MEMHEAP_SIZE
#define BLE_MEMHEAP_SIZE (1024 * 6)
#endif
__attribute__((aligned(4))) u32 MEM_BUF[BLE_MEMHEAP_SIZE / 4];
#ifndef BLE_BUFF_MAX_LEN
#define BLE_BUFF_MAX_LEN 128 + 5
#endif
#ifndef BLE_BUFF_NUM
#define BLE_BUFF_NUM 5
#endif
#ifndef BLE_TX_NUM_EVENT
#define BLE_TX_NUM_EVENT 1
#endif
#ifndef BLE_TX_POWER
#define BLE_TX_POWER LL_TX_POWEER_0_DBM
#endif
#ifndef PERIPHERAL_MAX_CONNECTION
#define PERIPHERAL_MAX_CONNECTION 1
#endif
#ifndef CENTRAL_MAX_CONNECTION
#define CENTRAL_MAX_CONNECTION 3
#endif

uint32_t sys_random();
void BleInit(uint8_t *mac, uint8_t tx_power) {
  uint8_t i;
  bleConfig_t cfg;
  if (tmos_memcmp(VER_LIB, VER_FILE, strlen(VER_FILE)) == FALSE) {
    while (1)
      ;
  }

  tmos_memset(&cfg, 0, sizeof(bleConfig_t));
  cfg.MEMAddr = (intptr_t)MEM_BUF;
  cfg.MEMLen = (uint16_t)BLE_MEMHEAP_SIZE;
  cfg.BufNumber = (uint8_t)BLE_BUFF_NUM;
  cfg.BufMaxLen = (uint16_t)BLE_BUFF_MAX_LEN;
  cfg.TxNumEvent = (uint8_t)BLE_TX_NUM_EVENT;
  cfg.TxPower = tx_power;
  cfg.SelRTCClock = (uint8_t)1;
  cfg.ConnectNumber =
      (PERIPHERAL_MAX_CONNECTION & 3) | (CENTRAL_MAX_CONNECTION << 2);
  cfg.srandCB = sys_random;
  {
    for (i = 0; i < 6; i++) {
      cfg.MacAddr[i] = mac[i]; // Ê¹ÓÃÐ¾Æ¬macµØÖ·
    }
  }
  if (!cfg.MEMAddr || cfg.MEMLen < 4 * 1024) {
    while (1)
      ;
  }
  i = BLE_LibInit(&cfg);
  if (i) {
    while (1)
      ;
  }
}

static gapRolesBroadcasterCBs_t Broadcaster_BroadcasterCBs = {
    NULL, // Not used in peripheral role
    NULL  // Receive scan request callback
};
void disable_broadcast() {
  GAPRole_BroadcasterSetCB(&Broadcaster_BroadcasterCBs);
}

void ble_state_notify_callback_internal(gapRole_States_t newState,
                                        uint8_t *opcode);
static void ble_state_notify_callback(gapRole_States_t newState,
                                      gapRoleEvent_t *pEvent) {
  if (pEvent) {
    ble_state_notify_callback_internal(newState, &pEvent->gap.opcode);
  } else {
    ble_state_notify_callback_internal(newState, NULL);
  }
}

void ble_passcode_callback_internal(uint8_t *deviceAddr,
                                    uint16_t connectionHandle, uint8_t uiInputs,
                                    uint8_t uiOutputs);
void ble_pair_state_callback_internal(uint16_t connectionHandle, uint8_t state,
                                      uint8_t status);
void ble_oob_callback_internal(uint8_t *deviceAddr, uint16_t connectionHandle,
                               uint8_t *r_local, uint8_t *c_local);
static gapBondCBs_t Peripheral_BondMgrCBs = {
    ble_passcode_callback_internal,
    ble_pair_state_callback_internal,
    ble_oob_callback_internal,
};

void ble_rssi_read_callback_internal(uint16_t connHandle, int8_t newRSSI);
void ble_param_update_callback_internal(uint16_t connHandle,
                                        uint16_t connInterval,
                                        uint16_t connSlaveLatency,
                                        uint16_t connTimeout);
static gapRolesCBs_t Peripheral_PeripheralCBs = {
    ble_state_notify_callback,
    ble_rssi_read_callback_internal,
    ble_param_update_callback_internal,
};
uint16_t rust_process_event(uint8_t task_id, uint16_t events);
void start_device() {
  uint8 task_id = TMOS_ProcessEventRegister(rust_process_event);
  GAPRole_PeripheralStartDevice(task_id, &Peripheral_BondMgrCBs,
                                &Peripheral_PeripheralCBs);
}
uint32_t get_mac_address(uint8_t *buf) { return GetMACAddress(buf); };
