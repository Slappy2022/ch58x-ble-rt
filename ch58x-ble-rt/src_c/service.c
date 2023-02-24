#include "service.h"

#include <string.h>

#include "CH58xBLE_LIB.h"

#define ALIGNMENT 4

// rust forward defs
uint8_t *rust_alloc(size_t len, size_t align);

uint8_t ble_read_callback_internal(uint16_t connHandle, gattAttribute_t *pAttr,
                                   uint8_t *pValue, uint16_t *pLen,
                                   uint16_t offset, uint16_t maxLen,
                                   uint8_t method);
uint8_t ble_write_callback_internal(uint16_t connHandle, gattAttribute_t *pAttr,
                                    uint8_t *pValue, uint16_t len,
                                    uint16_t offset, uint8_t method);
uint8_t ble_auth_callback_internal(uint16_t connHandle, gattAttribute_t *pAttr,
                                   uint8_t opcode);

static gattServiceCBs_t appCBs = {
    ble_read_callback_internal,
    ble_write_callback_internal,
    ble_auth_callback_internal,
};

// Internal structs
typedef struct uuid_t {
  uint8_t uuid[16];
} uuid_t;

typedef struct characteristic_t {
  uuid_t uuid;
  uint8_t properties;
} characteristic_t;

typedef struct service_t {
  uuid_t serviceUuid;
  gattAttrType_t serviceType;
  size_t attributesLen;
  gattAttribute_t *attributes;
  size_t characteristicsLen;
  characteristic_t *characteristics;
} service_t;

uint8_t register_service(service_ptr_t *ptr, const service_info_t *info) {
  service_t *service = (service_t *)rust_alloc(sizeof(service_t), ALIGNMENT);

  // Make space for attributes
  size_t attributesLen = 1 + 2 * info->characteristicsLen;
  gattAttribute_t *attributes = (gattAttribute_t *)rust_alloc(
      attributesLen * sizeof(gattAttribute_t), ALIGNMENT);

  // Container for service attribute references
  service->attributesLen = attributesLen;
  service->attributes = attributes;
  memcpy(service->serviceUuid.uuid, info->uuid, sizeof(info->uuid));
  uint8_t uuidSize = info->is_short != 0 ? ATT_BT_UUID_SIZE : ATT_UUID_SIZE;
  gattAttrType_t serviceType = {uuidSize, &service->serviceUuid.uuid[0]};
  service->serviceType = serviceType;

  // Make space for characteristic references
  characteristic_t *characteristics = (characteristic_t *)rust_alloc(
      info->characteristicsLen * sizeof(characteristic_t), ALIGNMENT);
  service->characteristics = characteristics;
  service->characteristicsLen = info->characteristicsLen;

  // Set service attribute
  gattAttribute_t service_attribute = {{ATT_BT_UUID_SIZE, primaryServiceUUID},
                                       GATT_PERMIT_READ,
                                       0,
                                       (uint8 *)&service->serviceType};
  service->attributes[0] = service_attribute;

  // Set characteristic attributes
  for (size_t i = 0; i < service->characteristicsLen; i++) {
    // Copy over stuff from info that needs references
    memcpy(service->characteristics[i].uuid.uuid, info->characteristics[i].uuid,
           sizeof(info->characteristics[i].uuid));
    service->characteristics[i].properties =
        info->characteristics[i].properties;

    // Create the pair of attributes for the characteristic
    gattAttribute_t description = {{ATT_BT_UUID_SIZE, characterUUID},
                                   GATT_PERMIT_READ,
                                   0,
                                   &service->characteristics[i].properties};
    service->attributes[1 + i * 2] = description;

    uint8_t uuidSize = info->characteristics[i].is_short != 0 ? ATT_BT_UUID_SIZE
                                                              : ATT_UUID_SIZE;
    gattAttribute_t definition = {
        {uuidSize, &service->characteristics[i].uuid.uuid[0]},
        info->characteristics[i].permissions,
        0,
        NULL};
    service->attributes[2 + i * 2] = definition;
  }

  ptr->ptr = service;

  return GATTServApp_RegisterService(service->attributes,
                                     service->attributesLen,
                                     GATT_MAX_ENCRYPT_KEY_SIZE, &appCBs);
}
