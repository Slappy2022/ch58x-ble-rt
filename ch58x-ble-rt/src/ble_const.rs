#[allow(non_camel_case_types, unused)]
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum TxPower {
    LL_TX_POWEER_MINUS_16_DBM = 0x01,
    LL_TX_POWEER_MINUS_12_DBM = 0x02,
    LL_TX_POWEER_MINUS_8_DBM = 0x04,
    LL_TX_POWEER_MINUS_5_DBM = 0x07,
    LL_TX_POWEER_MINUS_3_DBM = 0x09,
    LL_TX_POWEER_MINUS_1_DBM = 0x0B,
    LL_TX_POWEER_0_DBM = 0x0D,
    LL_TX_POWEER_1_DBM = 0x0F,
    LL_TX_POWEER_2_DBM = 0x13,
    LL_TX_POWEER_3_DBM = 0x17,
    LL_TX_POWEER_4_DBM = 0x1D,
    LL_TX_POWEER_5_DBM = 0x29,
    LL_TX_POWEER_6_DBM = 0x3D,
}

pub static ERR_LLE_IRQ_HANDLE: u8 = 0x01;
pub static ERR_MEM_ALLOCATE_SIZE: u8 = 0x02;
pub static ERR_SET_MAC_ADDR: u8 = 0x03;
pub static ERR_GAP_ROLE_CONFIG: u8 = 0x04;
pub static ERR_CONNECT_NUMBER_CONFIG: u8 = 0x05;
pub static ERR_SNV_ADDR_CONFIG: u8 = 0x06;
pub static ERR_CLOCK_SELECT_CONFIG: u8 = 0x07;

/*
pub static B_ADDR_LEN                     :u8= 6;

pub static B_RANDOM_NUM_SIZE              :u8= 8;

pub static KEYLEN                         :u8= 16;
pub static PUBLIC_KEY_LEN                 :u8= 64;


pub static B_MAX_ADV_LEN                  :u8= 31;
pub static B_MAX_ADV_EXT_LEN              :u8= 460;
pub static B_MAX_ADV_PERIODIC_LEN         :u8= 460;

pub static FAILURE                         :u8=0x01;
pub static INVALIDPARAMETER                :u8=0x02;
pub static INVALID_TASK                    :u8=0x03;
pub static MSG_BUFFER_NOT_AVAIL            :u8=0x04;
pub static INVALID_MSG_POINTER             :u8=0x05;
pub static INVALID_EVENT_ID                :u8=0x06;
pub static INVALID_TIMEOUT                 :u8=0x07;
pub static NO_TIMER_AVAIL                  :u8=0x08;
pub static NV_OPER_FAILED                  :u8=0x0A;
pub static INVALID_MEM_SIZE                :u8=0x0B;


pub static bleEecKeyRequestRejected        :u8=0x06;
pub static bleNotReady                     :u8=0x10;
pub static bleAlreadyInRequestedMode       :u8=0x11;
pub static bleIncorrectMode                :u8=0x12;
pub static bleMemAllocError                :u8=0x13;
pub static bleNotConnected                 :u8=0x14;
pub static bleNoResources                  :u8=0x15;
pub static blePending                      :u8=0x16;
pub static bleTimeout                      :u8=0x17;
pub static bleInvalidRange                 :u8=0x18;
pub static bleLinkEncrypted                :u8=0x19;
pub static bleProcedureComplete            :u8=0x1A;
pub static bleInvalidMtuSize               :u8=0x1B;


pub static INVALID_CONNHANDLE              :u8=0xFFFF;
pub static LOOPBACK_CONNHANDLE             :u8=0xFFFE;

pub static LINK_NOT_CONNECTED              :u8=0x00;
pub static LINK_CONNECTED                  :u8=0x01;
pub static LINK_AUTHENTICATED              :u8=0x02;
pub static LINK_BOUND                      :u8=0x04;
pub static LINK_ENCRYPTED                  :u8=0x10;

pub static LINKDB_STATUS_UPDATE_NEW       :u8= 0;
pub static LINKDB_STATUS_UPDATE_REMOVED   :u8= 1;
pub static LINKDB_STATUS_UPDATE_STATEFLAGS:u8= 2;
pub static GAP_SERVICE_UUID                :u8=0x1800;
pub static GATT_SERVICE_UUID               :u8=0x1801;

pub static GATT_PRIMARY_SERVICE_UUID       :u8=0x2800;
pub static GATT_SECONDARY_SERVICE_UUID     :u8=0x2801;
pub static GATT_INCLUDE_UUID               :u8=0x2802;
pub static GATT_CHARACTER_UUID             :u8=0x2803;


 * GATT Descriptors;
 *
pub static GATT_CHAR_EXT_PROPS_UUID        :u8=0x2900;
pub static GATT_CHAR_USER_DESC_UUID        :u8=0x2901;
pub static GATT_CLIENT_CHAR_CFG_UUID       :u8=0x2902;
pub static GATT_SERV_CHAR_CFG_UUID         :u8=0x2903;
pub static GATT_CHAR_FORMAT_UUID           :u8=0x2904;
pub static GATT_CHAR_AGG_FORMAT_UUID       :u8=0x2905;
pub static GATT_VALID_RANGE_UUID           :u8=0x2906;
pub static GATT_EXT_REPORT_REF_UUID        :u8=0x2907;
pub static GATT_REPORT_REF_UUID            :u8=0x2908;


 * GATT Characteristics;
 *
pub static DEVICE_NAME_UUID                :u8=0x2A00;
pub static APPEARANCE_UUID                 :u8=0x2A01;
pub static PERI_PRIVACY_FLAG_UUID          :u8=0x2A02;
pub static RECONNECT_ADDR_UUID             :u8=0x2A03;
pub static PERI_CONN_PARAM_UUID            :u8=0x2A04;
pub static SERVICE_CHANGED_UUID            :u8=0x2A05;
pub static CENTRAL_ADDRESS_RESOLUTION_UUID :u8=0x2AA6;


 * GATT Service UUIDs;
 *
pub static IMMEDIATE_ALERT_SERV_UUID       :u8=0x1802;
pub static LINK_LOSS_SERV_UUID             :u8=0x1803;
pub static TX_PWR_LEVEL_SERV_UUID          :u8=0x1804;
pub static CURRENT_TIME_SERV_UUID          :u8=0x1805;
pub static REF_TIME_UPDATE_SERV_UUID       :u8=0x1806;
pub static NEXT_DST_CHANGE_SERV_UUID       :u8=0x1807;
pub static GLUCOSE_SERV_UUID               :u8=0x1808;
pub static THERMOMETER_SERV_UUID           :u8=0x1809;
pub static DEVINFO_SERV_UUID               :u8=0x180A;
pub static NWA_SERV_UUID                   :u8=0x180B;
pub static HEARTRATE_SERV_UUID             :u8=0x180D;
pub static PHONE_ALERT_STS_SERV_UUID       :u8=0x180E;
pub static BATT_SERV_UUID                  :u8=0x180F;
pub static BLOODPRESSURE_SERV_UUID         :u8=0x1810;
pub static ALERT_NOTIF_SERV_UUID           :u8=0x1811;
pub static HID_SERV_UUID                   :u8=0x1812;
pub static SCAN_PARAM_SERV_UUID            :u8=0x1813;
pub static RSC_SERV_UUID                   :u8=0x1814;
pub static CSC_SERV_UUID                   :u8=0x1816;
pub static CYCPWR_SERV_UUID                :u8=0x1818;
pub static LOC_NAV_SERV_UUID               :u8=0x1819;


 * GATT Characteristic UUIDs;
 *
pub static ALERT_LEVEL_UUID                :u8=0x2A06;
pub static TX_PWR_LEVEL_UUID               :u8=0x2A07;
pub static DATE_TIME_UUID                  :u8=0x2A08;
pub static DAY_OF_WEEK_UUID                :u8=0x2A09;
pub static DAY_DATE_TIME_UUID              :u8=0x2A0A;
pub static EXACT_TIME_256_UUID             :u8=0x2A0C;
pub static DST_OFFSET_UUID                 :u8=0x2A0D;
pub static TIME_ZONE_UUID                  :u8=0x2A0E;
pub static LOCAL_TIME_INFO_UUID            :u8=0x2A0F;
pub static TIME_WITH_DST_UUID              :u8=0x2A11;
pub static TIME_ACCURACY_UUID              :u8=0x2A12;
pub static TIME_SOURCE_UUID                :u8=0x2A13;
pub static REF_TIME_INFO_UUID              :u8=0x2A14;
pub static TIME_UPDATE_CTRL_PT_UUID        :u8=0x2A16;
pub static TIME_UPDATE_STATE_UUID          :u8=0x2A17;
pub static GLUCOSE_MEAS_UUID               :u8=0x2A18;
pub static BATT_LEVEL_UUID                 :u8=0x2A19;
pub static TEMP_MEAS_UUID                  :u8=0x2A1C;
pub static TEMP_TYPE_UUID                  :u8=0x2A1D;
pub static IMEDIATE_TEMP_UUID              :u8=0x2A1E;
pub static MEAS_INTERVAL_UUID              :u8=0x2A21;
pub static BOOT_KEY_INPUT_UUID             :u8=0x2A22;
pub static SYSTEM_ID_UUID                  :u8=0x2A23;
pub static MODEL_NUMBER_UUID               :u8=0x2A24;
pub static SERIAL_NUMBER_UUID              :u8=0x2A25;
pub static FIRMWARE_REV_UUID               :u8=0x2A26;
pub static HARDWARE_REV_UUID               :u8=0x2A27;
pub static SOFTWARE_REV_UUID               :u8=0x2A28;
pub static MANUFACTURER_NAME_UUID          :u8=0x2A29;
pub static IEEE_11073_CERT_DATA_UUID       :u8=0x2A2A;
pub static CURRENT_TIME_UUID               :u8=0x2A2B;
pub static SCAN_REFRESH_UUID               :u8=0x2A31;
pub static BOOT_KEY_OUTPUT_UUID            :u8=0x2A32;
pub static BOOT_MOUSE_INPUT_UUID           :u8=0x2A33;
pub static GLUCOSE_CONTEXT_UUID            :u8=0x2A34;
pub static BLOODPRESSURE_MEAS_UUID         :u8=0x2A35;
pub static IMEDIATE_CUFF_PRESSURE_UUID     :u8=0x2A36;
pub static HEARTRATE_MEAS_UUID             :u8=0x2A37;
pub static BODY_SENSOR_LOC_UUID            :u8=0x2A38;
pub static HEARTRATE_CTRL_PT_UUID          :u8=0x2A39;
pub static NETWORK_AVAIL_UUID              :u8=0x2A3E;
pub static ALERT_STATUS_UUID               :u8=0x2A3F;
pub static RINGER_CTRL_PT_UUID             :u8=0x2A40;
pub static RINGER_SETTING_UUID             :u8=0x2A41;
pub static ALERT_CAT_ID_BMASK_UUID         :u8=0x2A42;
pub static ALERT_CAT_ID_UUID               :u8=0x2A43;
pub static ALERT_NOTIF_CTRL_PT_UUID        :u8=0x2A44;
pub static UNREAD_ALERT_STATUS_UUID        :u8=0x2A45;
pub static NEW_ALERT_UUID                  :u8=0x2A46;
pub static SUP_NEW_ALERT_CAT_UUID          :u8=0x2A47;
pub static SUP_UNREAD_ALERT_CAT_UUID       :u8=0x2A48;
pub static BLOODPRESSURE_FEATURE_UUID      :u8=0x2A49;
pub static HID_INFORMATION_UUID            :u8=0x2A4A;
pub static REPORT_MAP_UUID                 :u8=0x2A4B;
pub static HID_CTRL_PT_UUID                :u8=0x2A4C;
pub static REPORT_UUID                     :u8=0x2A4D;
pub static PROTOCOL_MODE_UUID              :u8=0x2A4E;
pub static SCAN_INTERVAL_WINDOW_UUID       :u8=0x2A4F;
pub static PNP_ID_UUID                     :u8=0x2A50;
pub static GLUCOSE_FEATURE_UUID            :u8=0x2A51;
pub static RECORD_CTRL_PT_UUID             :u8=0x2A52;
pub static RSC_MEAS_UUID                   :u8=0x2A53;
pub static RSC_FEATURE_UUID                :u8=0x2A54;
pub static SC_CTRL_PT_UUID                 :u8=0x2A55;
pub static CSC_MEAS_UUID                   :u8=0x2A5B;
pub static CSC_FEATURE_UUID                :u8=0x2A5C;
pub static SENSOR_LOC_UUID                 :u8=0x2A5D;
pub static CYCPWR_MEAS_UUID                :u8=0x2A63;
pub static CYCPWR_VECTOR_UUID              :u8=0x2A64;
pub static CYCPWR_FEATURE_UUID             :u8=0x2A65;
pub static CYCPWR_CTRL_PT_UUID             :u8=0x2A66;
pub static LOC_SPEED_UUID                  :u8=0x2A67;
pub static NAV_UUID                        :u8=0x2A68;
pub static POS_QUALITY_UUID                :u8=0x2A69;
pub static LN_FEATURE_UUID                 :u8=0x2A6A;
pub static LN_CTRL_PT_UUID                 :u8=0x2A6B;
pub static ELE_UUID                        :u8=0x2A6C;
pub static PRESSURE_UUID                   :u8=0x2A6D;
pub static TEMP_UUID                       :u8=0x2A6E;
pub static HUMI_UUID                       :u8=0x2A6F;
pub static TRUE_WIND_SPEED_UUID            :u8=0x2A70;
pub static TRUE_WIND_DIRECTION_UUID        :u8=0x2A71;
pub static URI_UUID                        :u8=0x2AB6;
pub static MEDIA_STATE_UUID                :u8=0x2BA3;
pub static MEDIA_CTRL_PT_UUID              :u8=0x2BA4;
pub static MEDIA_CTRL_PT_OS_UUID           :u8=0x2BA5;
pub static CALL_STATE_UUID                 :u8=0x2BBD;
pub static CALL_CTRL_PT_UUID               :u8=0x2BBE;
pub static CALL_CTRL_PT_OO_UUID            :u8=0x2BBF;
pub static TERM_REASON_UUID                :u8=0x2BC0;
pub static INCOMING_CALL_UUID              :u8=0x2BC1;
pub static MUTE_UUID                       :u8=0x2BC3;


 * GATT Unit UUIDs;
 *
pub static GATT_UNITLESS_UUID                      :u8=0x2700;
pub static GATT_UNIT_LENGTH_METER_UUID             :u8=0x2701;
pub static GATT_UNIT_MASS_KGRAM_UUID               :u8=0x2702;
pub static GATT_UNIT_TIME_SECOND_UUID              :u8=0x2703;
pub static GATT_UNIT_ELECTRIC_CURRENT_A_UUID       :u8=0x2704;
pub static GATT_UNIT_THERMODYN_TEMP_K_UUID         :u8=0x2705;
pub static GATT_UNIT_AMOUNT_SUBSTANCE_M_UUID       :u8=0x2706;
pub static GATT_UNIT_LUMINOUS_INTENSITY_C_UUID     :u8=0x2707;

pub static GATT_UNIT_AREA_SQ_MTR_UUID              :u8=0x2710;
pub static GATT_UNIT_VOLUME_CUBIC_MTR_UUID         :u8=0x2711;
pub static GATT_UNIT_VELOCITY_MPS_UUID             :u8=0x2712;
pub static GATT_UNIT_ACCELERATION_MPS_SQ_UUID      :u8=0x2713;
pub static GATT_UNIT_WAVENUMBER_RM_UUID            :u8=0x2714;
pub static GATT_UNIT_DENSITY_KGPCM_UUID            :u8=0x2715;
pub static GATT_UNIT_SURFACE_DENSITY_KGPSM_UUID    :u8=0x2716;
pub static GATT_UNIT_SPECIFIC_VOLUME_CMPKG_UUID    :u8=0x2717;
pub static GATT_UNIT_CURRENT_DENSITY_APSM_UUID     :u8=0x2718;
pub static GATT_UNIT_MAG_FIELD_STRENGTH_UUID       :u8=0x2719;
pub static GATT_UNIT_AMOUNT_CONC_MPCM_UUID         :u8=0x271A;
pub static GATT_UNIT_MASS_CONC_KGPCM_UUID          :u8=0x271B;
pub static GATT_UNIT_LUMINANCE_CPSM_UUID           :u8=0x271C;
pub static GATT_UNIT_REFRACTIVE_INDEX_UUID         :u8=0x271D;
pub static GATT_UNIT_RELATIVE_PERMEABLILTY_UUID    :u8=0x271E;
pub static GATT_UNIT_PLANE_ANGLE_RAD_UUID          :u8=0x2720;
pub static GATT_UNIT_SOLID_ANGLE_STERAD_UUID       :u8=0x2721;
pub static GATT_UNIT_FREQUENCY_HTZ_UUID            :u8=0x2722;
pub static GATT_UNIT_FORCE_NEWTON_UUID             :u8=0x2723;
pub static GATT_UNIT_PRESSURE_PASCAL_UUID          :u8=0x2724;
pub static GATT_UNIT_ENERGY_JOULE_UUID             :u8=0x2725;
pub static GATT_UNIT_POWER_WATT_UUID               :u8=0x2726;
pub static GATT_UNIT_E_CHARGE_C_UUID               :u8=0x2727;
pub static GATT_UNIT_E_POTENTIAL_DIF_V_UUID        :u8=0x2728;

pub static GATT_UNIT_CELSIUS_TEMP_DC_UUID          :u8=0x272F;

pub static GATT_UNIT_TIME_MINUTE_UUID              :u8=0x2760;
pub static GATT_UNIT_TIME_HOUR_UUID                :u8=0x2761;
pub static GATT_UNIT_TIME_DAY_UUID                 :u8=0x2762;
pub static GATT_UNIT_PLANE_ANGLE_DEGREE_UUID       :u8=0x2763;
pub static GATT_UNIT_PLANE_ANGLE_MINUTE_UUID       :u8=0x2764;
pub static GATT_UNIT_PLANE_ANGLE_SECOND_UUID       :u8=0x2765;
pub static GATT_UNIT_AREA_HECTARE_UUID             :u8=0x2766;
pub static GATT_UNIT_VOLUME_LITRE_UUID             :u8=0x2767;
pub static GATT_UNIT_MASS_TONNE_UUID               :u8=0x2768;

pub static GATT_UINT_LENGTH_YARD_UUID              :u8=0x27A0;
pub static GATT_UNIT_LENGTH_PARSEC_UUID            :u8=0x27A1;
pub static GATT_UNIT_LENGTH_INCH_UUID              :u8=0x27A2;
pub static GATT_UNIT_LENGTH_FOOT_UUID              :u8=0x27A3;
pub static GATT_UNIT_LENGTH_MILE_UUID              :u8=0x27A4;
pub static GATT_UNIT_PRESSURE_PFPSI_UUID           :u8=0x27A5;
pub static GATT_UNIT_VELOCITY_KMPH_UUID            :u8=0x27A6;
pub static GATT_UNIT_VELOCITY_MPH_UUID             :u8=0x27A7;
pub static GATT_UNIT_ANGULAR_VELOCITY_RPM_UUID     :u8=0x27A8;
pub static GATT_UNIT_ENERGY_GCAL_UUID              :u8=0x27A9;
pub static GATT_UNIT_ENERGY_KCAL_UUID              :u8=0x27AA;
pub static GATT_UNIT_ENERGY_KWH_UUID               :u8=0x27AB;
pub static GATT_UNIT_THERMODYN_TEMP_DF_UUID        :u8=0x27AC;
pub static GATT_UNIT_PERCENTAGE_UUID               :u8=0x27AD;
pub static GATT_UNIT_PER_MILE_UUID                 :u8=0x27AE;
pub static GATT_UNIT_PERIOD_BPM_UUID               :u8=0x27AF;
pub static GATT_UNIT_E_CHARGE_AH_UUID              :u8=0x27B0;
pub static GATT_UNIT_MASS_DENSITY_MGPD_UUID        :u8=0x27B1;
pub static GATT_UNIT_MASS_DENSITY_MMPL_UUID        :u8=0x27B2;
pub static GATT_UNIT_TIME_YEAR_UUID                :u8=0x27B3;
pub static GATT_UNIT_TIME_MONTH_UUID               :u8=0x27B4;



pub static GATT_MSG_EVENT                  :u8=0xB0;
pub static GATT_SERV_MSG_EVENT             :u8=0xB1;

pub static GAP_MSG_EVENT                   :u8=0xD0;

pub static ATT_MTU_SIZE                   :u8= 23;
pub static ATT_MAX_MTU_SIZE               :u8= 512;

pub static ATT_ERROR_RSP                   :u8=0x01;
pub static ATT_EXCHANGE_MTU_REQ            :u8=0x02;
pub static ATT_EXCHANGE_MTU_RSP            :u8=0x03;
pub static ATT_FIND_INFO_REQ               :u8=0x04;
pub static ATT_FIND_INFO_RSP               :u8=0x05;
pub static ATT_FIND_BY_TYPE_VALUE_REQ      :u8=0x06;
pub static ATT_FIND_BY_TYPE_VALUE_RSP      :u8=0x07;
pub static ATT_READ_BY_TYPE_REQ            :u8=0x08;
pub static ATT_READ_BY_TYPE_RSP            :u8=0x09;
pub static ATT_READ_REQ                    :u8=0x0a;
pub static ATT_READ_RSP                    :u8=0x0b;
pub static ATT_READ_BLOB_REQ               :u8=0x0c;
pub static ATT_READ_BLOB_RSP               :u8=0x0d;
pub static ATT_READ_MULTI_REQ              :u8=0x0e;
pub static ATT_READ_MULTI_RSP              :u8=0x0f;
pub static ATT_READ_BY_GRP_TYPE_REQ        :u8=0x10;
pub static ATT_READ_BY_GRP_TYPE_RSP        :u8=0x11;
pub static ATT_WRITE_REQ                   :u8=0x12;
pub static ATT_WRITE_RSP                   :u8=0x13;
pub static ATT_PREPARE_WRITE_REQ           :u8=0x16;
pub static ATT_PREPARE_WRITE_RSP           :u8=0x17;
pub static ATT_EXECUTE_WRITE_REQ           :u8=0x18;
pub static ATT_EXECUTE_WRITE_RSP           :u8=0x19;
pub static ATT_HANDLE_VALUE_NOTI           :u8=0x1b;
pub static ATT_HANDLE_VALUE_IND            :u8=0x1d;
pub static ATT_HANDLE_VALUE_CFM            :u8=0x1e;

pub static ATT_WRITE_CMD                   :u8=0x52;
pub static ATT_SIGNED_WRITE_CMD            :u8=0xD2;
*/

#[allow(non_camel_case_types, unused)]
#[repr(u8)]
pub enum AttributeError {
    ATT_ERR_INVALID_HANDLE = 0x01,
    ATT_ERR_READ_NOT_PERMITTED = 0x02,
    ATT_ERR_WRITE_NOT_PERMITTED = 0x03,
    ATT_ERR_INVALID_PDU = 0x04,
    ATT_ERR_INSUFFICIENT_AUTHEN = 0x05,
    ATT_ERR_UNSUPPORTED_REQ = 0x06,
    ATT_ERR_INVALID_OFFSET = 0x07,
    ATT_ERR_INSUFFICIENT_AUTHOR = 0x08,
    ATT_ERR_PREPARE_QUEUE_FULL = 0x09,
    ATT_ERR_ATTR_NOT_FOUND = 0x0a,
    ATT_ERR_ATTR_NOT_LONG = 0x0b,
    ATT_ERR_INSUFFICIENT_KEY_SIZE = 0x0c,
    ATT_ERR_INVALID_VALUE_SIZE = 0x0d,
    ATT_ERR_UNLIKELY = 0x0e,
    ATT_ERR_INSUFFICIENT_ENCRYPT = 0x0f,
    ATT_ERR_UNSUPPORTED_GRP_TYPE = 0x10,
    ATT_ERR_INSUFFICIENT_RESOURCES = 0x11,
    ATT_ERR_INVALID_VALUE = 0x80,
}

/*
pub static ATT_FLOW_CTRL_VIOLATED_EVENT    :u8=0x7E;
pub static ATT_MTU_UPDATED_EVENT           :u8=0x7F;

pub static ATT_BT_UUID_SIZE               :u8= 2;

pub static ATT_UUID_SIZE                  :u8= 16;

*/

/*
pub static GATT_LOCAL_READ                 :u8=0xFF;
pub static GATT_LOCAL_WRITE                :u8=0xFE;


pub static GATT_MIN_ENCRYPT_KEY_SIZE      :u8= 7;
pub static GATT_MAX_ENCRYPT_KEY_SIZE      :u8= 16;


pub static GATT_INVALID_HANDLE             :u8=0x0000;
pub static GATT_MIN_HANDLE                 :u8=0x0001;
pub static GATT_MAX_HANDLE                 :u8=0xFFFF;

pub static GATT_MAX_MTU                    :u8=0xFFFF;


pub static gattPermitRead( a )             ( (a) & GATT_PERMIT_READ );
pub static gattPermitWrite( a )            ( (a) & GATT_PERMIT_WRITE );
pub static gattPermitAuthenRead( a )       ( (a) & GATT_PERMIT_AUTHEN_READ );
pub static gattPermitAuthenWrite( a )      ( (a) & GATT_PERMIT_AUTHEN_WRITE );
pub static gattPermitAuthorRead( a )       ( (a) & GATT_PERMIT_AUTHOR_READ );
pub static gattPermitAuthorWrite( a )      ( (a) & GATT_PERMIT_AUTHOR_WRITE );
pub static gattPermitEncryptRead( a )      ( (a) & GATT_PERMIT_ENCRYPT_READ );
pub static gattPermitEncryptWrite( a )     ( (a) & GATT_PERMIT_ENCRYPT_WRITE );


pub static gattPrimaryServiceType( t )     ( ATT_CompareUUID( primaryServiceUUID, ATT_BT_UUID_SIZE, (t).uuid, (t).len ) );
pub static gattSecondaryServiceType( t )   ( ATT_CompareUUID( secondaryServiceUUID, ATT_BT_UUID_SIZE, (t).uuid, (t).len ) );
pub static gattCharacterType( t )          ( ATT_CompareUUID( characterUUID, ATT_BT_UUID_SIZE, (t).uuid, (t).len ) );
pub static gattIncludeType( t )            ( ATT_CompareUUID( includeUUID, ATT_BT_UUID_SIZE, (t).uuid, (t).len ) );
pub static gattServiceType( t )            ( gattPrimaryServiceType( (t) ) || gattSecondaryServiceType( (t) ) );
pub static GATT_MAX_NUM_CONN               (4);


pub static GATT_CLIENT_CFG_NOTIFY          :u8=0x0001;
pub static GATT_CLIENT_CFG_INDICATE        :u8=0x0002;

pub static GATT_CFG_NO_OPERATION           :u8=0x0000;


*/
pub static GATT_ALL_SERVICES: u32 = 0xFFFFFFFF;
/*


pub static GATT_NUM_ATTRS( attrs )         ( sizeof( attrs )


pub static GATT_SERVICE_HANDLE( attrs )    ( (attrs)[0].handle );


pub static GATT_INCLUDED_HANDLE( attrs, i ) ( *((uint16_t *)((attrs)[(i)].pValue)) );


pub static GATT_CCC_TBL( pValue )          ( (gattCharCfg_t *)(*((PTR_TYPE)(&pValue))));

*/

#[allow(non_camel_case_types, unused)]
#[derive(Clone, Copy, Debug, int_enum::IntEnum)]
#[repr(u8)]
pub enum GapMessageEvent {
    GAP_DEVICE_INIT_DONE_EVENT = 0x00,
    GAP_DEVICE_DISCOVERY_EVENT = 0x01,
    GAP_ADV_DATA_UPDATE_DONE_EVENT = 0x02,
    GAP_MAKE_DISCOVERABLE_DONE_EVENT = 0x03,
    GAP_END_DISCOVERABLE_DONE_EVENT = 0x04,
    GAP_LINK_ESTABLISHED_EVENT = 0x05,
    GAP_LINK_TERMINATED_EVENT = 0x06,
    GAP_LINK_PARAM_UPDATE_EVENT = 0x07,
    GAP_RANDOM_ADDR_CHANGED_EVENT = 0x08,
    GAP_SIGNATURE_UPDATED_EVENT = 0x09,
    GAP_AUTHENTICATION_COMPLETE_EVENT = 0x0A,
    GAP_PASSKEY_NEEDED_EVENT = 0x0B,
    GAP_SLAVE_REQUESTED_SECURITY_EVENT = 0x0C,
    GAP_DEVICE_INFO_EVENT = 0x0D,
    GAP_BOND_COMPLETE_EVENT = 0x0E,
    GAP_PAIRING_REQ_EVENT = 0x0F,
    GAP_DIRECT_DEVICE_INFO_EVENT = 0x10,
    GAP_PHY_UPDATE_EVENT = 0x11,
    GAP_EXT_ADV_DEVICE_INFO_EVENT = 0x12,
    GAP_MAKE_PERIODIC_ADV_DONE_EVENT = 0x13,
    GAP_END_PERIODIC_ADV_DONE_EVENT = 0x14,
    GAP_SYNC_ESTABLISHED_EVENT = 0x15,
    GAP_PERIODIC_ADV_DEVICE_INFO_EVENT = 0x16,
    GAP_SYNC_LOST_EVENT = 0x17,
    GAP_SCAN_REQUEST_EVENT = 0x19,
    GAP_OOB_NEEDED_EVENT = 0x1A,
    GAP_MAKE_CONNECTIONESS_CTE_DONE_EVENT = 0x1B,
    GAP_END_CONNECTIONESS_CTE_DONE_EVENT = 0x1C,
    GAP_PERI_ADV_SYNC_TRAN_RECEIVED_EVENT = 0x1D,
}
/*


pub static GAP_PROFILE_BROADCASTER                 :u8=0x01;
pub static GAP_PROFILE_OBSERVER                    :u8=0x02;
pub static GAP_PROFILE_PERIPHERAL                  :u8=0x04;
pub static GAP_PROFILE_CENTRAL                     :u8=0x08;


pub static bleGAPUserCanceled                      :u8=0x30;
pub static bleGAPConnNotAcceptable                 :u8=0x31;
pub static bleGAPBondRejected                      :u8=0x32;
pub static bleGAPExpiredCanceled                   :u8=0x33;

pub static GAP_DEVICE_NAME_LEN                    :u8= 21;


pub static LISTEN_PERIODIC_ADVERTISING_MODE        (1<<0);
pub static REPORTING_INITIALLY_DISABLED            (1<<1);
pub static DUPLICATE_FILTERING_INITIALLY_ENABLED   (1<<2);


pub static GAP_CONNHANDLE_INIT                     :u8=0xFFFE;
pub static GAP_CONNHANDLE_ALL                      :u8=0xFFFF;



pub static GAP_PRIVACY_DISABLED                    :u8=0x00;
pub static GAP_PRIVACY_ENABLED                     :u8=0x01;

*/

pub static GGS_DEVICE_NAME_ATT: u16 = 0;
pub static GGS_APPEARANCE_ATT: u16 = 1;
pub static GGS_PERI_PRIVACY_FLAG_ATT: u16 = 2;
pub static GGS_RECONNCT_ADDR_ATT: u16 = 3;
pub static GGS_PERI_CONN_PARAM_ATT: u16 = 4;
pub static GGS_PERI_PRIVACY_FLAG_PROPS: u16 = 5;
pub static GGS_W_PERMIT_DEVICE_NAME_ATT: u16 = 6;
pub static GGS_W_PERMIT_APPEARANCE_ATT: u16 = 7;
pub static GGS_W_PERMIT_PRIVACY_FLAG_ATT: u16 = 8;
pub static GGS_CENT_ADDR_RES_ATT: u16 = 9;

/*
pub static GAP_SERVICE                             :u8=0x00000001;



pub static TGAP_GEN_DISC_ADV_MIN                  :u8= 0;
pub static TGAP_LIM_ADV_TIMEOUT                   :u8= 1;
pub static TGAP_DISC_SCAN                         :u8= 2;


pub static TGAP_DISC_ADV_INT_MIN                  :u8= 3;
pub static TGAP_DISC_ADV_INT_MAX                  :u8= 4;
pub static TGAP_DISC_SCAN_INT                     :u8= 5;
pub static TGAP_DISC_SCAN_WIND                    :u8= 6;


pub static TGAP_CONN_EST_INT_MIN                  :u8= 7;
pub static TGAP_CONN_EST_INT_MAX                  :u8= 8;
pub static TGAP_CONN_EST_SCAN_INT                 :u8= 9;
pub static TGAP_CONN_EST_SCAN_WIND                :u8= 10;
pub static TGAP_CONN_EST_HIGH_SCAN_INT            :u8= 11;
pub static TGAP_CONN_EST_HIGH_SCAN_WIND           :u8= 12;
pub static TGAP_CONN_EST_SUPERV_TIMEOUT           :u8= 13;
pub static TGAP_CONN_EST_LATENCY                  :u8= 14;
pub static TGAP_CONN_EST_MIN_CE_LEN               :u8= 15;
pub static TGAP_CONN_EST_MAX_CE_LEN               :u8= 16;


pub static TGAP_PRIVATE_ADDR_INT                  :u8= 17;
pub static TGAP_SM_TIMEOUT                        :u8= 18;
pub static TGAP_SM_MIN_KEY_LEN                    :u8= 19;
pub static TGAP_SM_MAX_KEY_LEN                    :u8= 20;
pub static TGAP_FILTER_ADV_REPORTS                :u8= 21;
pub static TGAP_SCAN_RSSI_MIN                     :u8= 22;
pub static TGAP_REJECT_CONN_PARAMS                :u8= 23;
pub static TGAP_AUTH_TASK_ID                      :u8= 24;


pub static TGAP_ADV_TX_POWER                      :u8= 25;
pub static TGAP_ADV_PRIMARY_PHY                   :u8= 26;
pub static TGAP_ADV_SECONDARY_PHY                 :u8= 27;
pub static TGAP_ADV_SECONDARY_MAX_SKIP            :u8= 28;
pub static TGAP_ADV_ADVERTISING_SID               :u8= 29;
pub static TGAP_ADV_SCAN_REQ_NOTIFY               :u8= 30;
pub static TGAP_ADV_ADVERTISING_DURATION          :u8= 31;
pub static TGAP_ADV_MAX_EVENTS                    :u8= 32;


pub static TGAP_DISC_SCAN_PHY                     :u8= 33;
pub static TGAP_DISC_SCAN_CODED_INT               :u8= 34;
pub static TGAP_DISC_SCAN_CODED_WIND              :u8= 35;
pub static TGAP_DISC_SCAN_DURATION                :u8= 36;
pub static TGAP_DISC_SCAN_PERIOD                  :u8= 37;


pub static TGAP_CONN_EST_INT_PHY                  :u8= 38;
pub static TGAP_CONN_EST_2M_INT_MIN               :u8= 39;
pub static TGAP_CONN_EST_2M_INT_MAX               :u8= 40;
pub static TGAP_CONN_EST_2M_SUPERV_TIMEOUT        :u8= 41;
pub static TGAP_CONN_EST_2M_LATENCY               :u8= 42;
pub static TGAP_CONN_EST_2M_MIN_CE_LEN            :u8= 43;
pub static TGAP_CONN_EST_2M_MAX_CE_LEN            :u8= 44;


pub static TGAP_CONN_EST_CODED_INT_MIN            :u8= 45;
pub static TGAP_CONN_EST_CODED_INT_MAX            :u8= 46;
pub static TGAP_CONN_EST_CODED_SCAN_INT           :u8= 47;
pub static TGAP_CONN_EST_CODED_SCAN_WIND          :u8= 48;
pub static TGAP_CONN_EST_CODED_HIGH_SCAN_INT      :u8= 49;
pub static TGAP_CONN_EST_CODED_HIGH_SCAN_WIND     :u8= 50;
pub static TGAP_CONN_EST_CODED_SUPERV_TIMEOUT     :u8= 51;
pub static TGAP_CONN_EST_CODED_LATENCY            :u8= 52;
pub static TGAP_CONN_EST_CODED_MIN_CE_LEN         :u8= 53;
pub static TGAP_CONN_EST_CODED_MAX_CE_LEN         :u8= 54;


pub static TGAP_PERIODIC_ADV_INT_MIN              :u8= 55;
pub static TGAP_PERIODIC_ADV_INT_MAX              :u8= 56;
pub static TGAP_PERIODIC_ADV_PROPERTIES           :u8= 57;

pub static TGAP_SCAN_MAX_LENGTH                   :u8= 58;
pub static TGAP_AFH_CHANNEL_MDOE                  :u8= 59;

pub static TGAP_PARAMID_MAX                       :u8= 60;


pub static DEVDISC_MODE_NONDISCOVERABLE            :u8=0x00;
pub static DEVDISC_MODE_GENERAL                    :u8=0x01;
pub static DEVDISC_MODE_LIMITED                    :u8=0x02;
pub static DEVDISC_MODE_ALL                        :u8=0x03;


pub static ADDRTYPE_PUBLIC                         :u8=0x00;
pub static ADDRTYPE_STATIC                         :u8=0x01;
pub static ADDRTYPE_PRIVATE_NONRESOLVE             :u8=0x02;
pub static ADDRTYPE_PRIVATE_RESOLVE                :u8=0x03;


pub static GAP_ADTYPE_ADV_IND                      :u8=0x00;
pub static GAP_ADTYPE_ADV_HDC_DIRECT_IND           :u8=0x01;
pub static GAP_ADTYPE_ADV_SCAN_IND                 :u8=0x02;
pub static GAP_ADTYPE_ADV_NONCONN_IND              :u8=0x03;
pub static GAP_ADTYPE_ADV_LDC_DIRECT_IND           :u8=0x04;

pub static GAP_ADTYPE_EXT_CONN_DIRECT              :u8=0x05;
pub static GAP_ADTYPE_EXT_SCAN_UNDIRECT            :u8=0x06;
pub static GAP_ADTYPE_EXT_NONCONN_NONSCAN_UNDIRECT :u8=0x07;
pub static GAP_ADTYPE_EXT_CONN_UNDIRECT            :u8=0x08;
pub static GAP_ADTYPE_EXT_SCAN_DIRECT              :u8=0x09;
pub static GAP_ADTYPE_EXT_NONCONN_NONSCAN_DIRECT   :u8=0x0A;


pub static GAP_PHY_VAL_TYPE;
pub static GAP_PHY_VAL_LE_1M                       :u8=0x01;
pub static GAP_PHY_VAL_LE_2M                       :u8=0x02;
pub static GAP_PHY_VAL_LE_CODED                    :u8=0x03;


pub static GAP_PHY_BIT_TYPE;
pub static GAP_PHY_BIT_LE_1M                       (1<<0);
pub static GAP_PHY_BIT_LE_2M                       (1<<1);
pub static GAP_PHY_BIT_LE_CODED                    (1<<2);
pub static GAP_PHY_BIT_ALL                         (GAP_PHY_BIT_LE_1M|GAP_PHY_BIT_LE_2M|GAP_PHY_BIT_LE_CODED);
pub static GAP_PHY_BIT_LE_CODED_S2                 (1<<3);


pub static GAP_PHY_OPTIONS_TYPE;
pub static GAP_PHY_OPTIONS_NOPRE                   :u8=0x00;
pub static GAP_PHY_OPTIONS_S2                      :u8=0x01;
pub static GAP_PHY_OPTIONS_S8                      :u8=0x02;


pub static GAP_PERI_PROPERTIES_INCLUDE_TXPOWER     (1<<6);


pub static GAP_ADVERTISEMENT_REPORT_TYPE_DEFINES;

pub static GAP_ADRPT_ADV_IND                       :u8=0x00;
pub static GAP_ADRPT_ADV_DIRECT_IND                :u8=0x01;
pub static GAP_ADRPT_ADV_SCAN_IND                  :u8=0x02;
pub static GAP_ADRPT_ADV_NONCONN_IND               :u8=0x03;
pub static GAP_ADRPT_SCAN_RSP                      :u8=0x04;
pub static GAP_ADRPT_EXT_CONN_DIRECT               :u8=0x05;
pub static GAP_ADRPT_EXT_SCAN_UNDIRECT             :u8=0x06;
pub static GAP_ADRPT_EXT_NONCONN_NONSCAN_UNDIRECT  :u8=0x07;
pub static GAP_ADRPT_EXT_CONN_UNDIRECT             :u8=0x08;
pub static GAP_ADRPT_EXT_SCAN_DIRECT               :u8=0x09;
pub static GAP_ADRPT_EXT_NONCONN_NONSCAN_DIRECT    :u8=0x0A;
pub static GAP_ADRPT_EXT_SCAN_RESPONSE             :u8=0x0B;

pub static GAP_ADRPT_EXT_DATA_MASK                 (3<<5);
pub static GAP_ADRPT_EXT_DATA_COMPLETE             (0<<5);
pub static GAP_ADRPT_EXT_DATA_INCOMPLETE           (1<<5);
pub static GAP_ADRPT_EXT_DATA_LAST                 (2<<5);


pub static GAP_ADRPT_ADV_CONNECTABLE               (1<<0);
pub static GAP_ADRPT_ADV_SCANNABLE                 (1<<1);
pub static GAP_ADRPT_ADV_DITECTED                  (1<<2);
pub static GAP_ADRPT_SCAN_RESPONSE                 (1<<3);


pub static GAP_FILTER_POLICY_ALL                   :u8=0x00;
pub static GAP_FILTER_POLICY_WHITE_SCAN            :u8=0x01;
pub static GAP_FILTER_POLICY_WHITE_CON             :u8=0x02;
pub static GAP_FILTER_POLICY_WHITE                 :u8=0x03;


pub static GAP_PASSCODE_MAX                       :u8= 999999;


pub static GAP_INIT_SIGN_COUNTER                   :u8=0xFFFFFFFF;


pub static GAP_ADVCHAN_37  :u8=0x01;
pub static GAP_ADVCHAN_38  :u8=0x02;
pub static GAP_ADVCHAN_39  :u8=0x04;
pub static GAP_ADVCHAN_ALL (GAP_ADVCHAN_37 | GAP_ADVCHAN_38 | GAP_ADVCHAN_39);
*/

pub static GAP_ADTYPE_FLAGS: u8 = 0x01;
pub static GAP_ADTYPE_16BIT_MORE: u8 = 0x02;
pub static GAP_ADTYPE_16BIT_COMPLETE: u8 = 0x03;
pub static GAP_ADTYPE_32BIT_MORE: u8 = 0x04;
pub static GAP_ADTYPE_32BIT_COMPLETE: u8 = 0x05;
pub static GAP_ADTYPE_128BIT_MORE: u8 = 0x06;
pub static GAP_ADTYPE_128BIT_COMPLETE: u8 = 0x07;
pub static GAP_ADTYPE_LOCAL_NAME_SHORT: u8 = 0x08;
pub static GAP_ADTYPE_LOCAL_NAME_COMPLETE: u8 = 0x09;
pub static GAP_ADTYPE_POWER_LEVEL: u8 = 0x0A;
pub static GAP_ADTYPE_OOB_CLASS_OF_DEVICE: u8 = 0x0D;
pub static GAP_ADTYPE_OOB_SIMPLE_PAIRING_HASHC: u8 = 0x0E;
pub static GAP_ADTYPE_OOB_SIMPLE_PAIRING_RANDR: u8 = 0x0F;
pub static GAP_ADTYPE_SM_TK: u8 = 0x10;
pub static GAP_ADTYPE_SM_OOB_FLAG: u8 = 0x11;
pub static GAP_ADTYPE_SLAVE_CONN_INTERVAL_RANGE: u8 = 0x12;
pub static GAP_ADTYPE_SIGNED_DATA: u8 = 0x13;
pub static GAP_ADTYPE_SERVICES_LIST_16BIT: u8 = 0x14;
pub static GAP_ADTYPE_SERVICES_LIST_128BIT: u8 = 0x15;
pub static GAP_ADTYPE_SERVICE_DATA: u8 = 0x16;
pub static GAP_ADTYPE_PUBLIC_TARGET_ADDR: u8 = 0x17;
pub static GAP_ADTYPE_RANDOM_TARGET_ADDR: u8 = 0x18;
pub static GAP_ADTYPE_APPEARANCE: u8 = 0x19;
pub static GAP_ADTYPE_ADV_INTERVAL: u8 = 0x1A;
pub static GAP_ADTYPE_LE_BD_ADDR: u8 = 0x1B;
pub static GAP_ADTYPE_LE_ROLE: u8 = 0x1C;
pub static GAP_ADTYPE_SIMPLE_PAIRING_HASHC_256: u8 = 0x1D;
pub static GAP_ADTYPE_SIMPLE_PAIRING_RANDR_256: u8 = 0x1E;
pub static GAP_ADTYPE_SERVICE_DATA_32BIT: u8 = 0x20;
pub static GAP_ADTYPE_SERVICE_DATA_128BIT: u8 = 0x21;
pub static GAP_ADTYPE_URI: u8 = 0x24;
pub static GAP_ADTYPE_INDOOR_POSITION: u8 = 0x25;
pub static GAP_ADTYPE_TRAN_DISCOVERY_DATA: u8 = 0x26;
pub static GAP_ADTYPE_SUPPORTED_FEATURES: u8 = 0x27;
pub static GAP_ADTYPE_CHANNEL_MAP_UPDATE: u8 = 0x28;
pub static GAP_ADTYPE_PB_ADV: u8 = 0x29;
pub static GAP_ADTYPE_MESH_MESSAGE: u8 = 0x2A;
pub static GAP_ADTYPE_MESH_BEACON: u8 = 0x2B;
pub static GAP_ADTYPE_BIG_INFO: u8 = 0x2C;
pub static GAP_ADTYPE_BROADCAST_CODE: u8 = 0x2D;
pub static GAP_ADTYPE_RSL_SET_IDENT: u8 = 0x2E;
pub static GAP_ADTYPE_ADV_INTERVAL_LONG: u8 = 0x2F;
pub static GAP_ADTYPE_3D_INFO_DATA: u8 = 0x3D;
pub static GAP_ADTYPE_MANUFACTURER_SPECIFIC: u8 = 0xFF;

pub static GAP_ADTYPE_FLAGS_LIMITED: u8 = 0x01;
pub static GAP_ADTYPE_FLAGS_GENERAL: u8 = 0x02;
pub static GAP_ADTYPE_FLAGS_BREDR_NOT_SUPPORTED: u8 = 0x04;

/*
pub static GAP_APPEARE_UNKNOWN                     :u8=0x0000;
pub static GAP_APPEARE_GENERIC_PHONE               :u8=0x0040;
pub static GAP_APPEARE_GENERIC_COMPUTER            :u8=0x0080;
pub static GAP_APPEARE_GENERIC_WATCH               :u8=0x00C0;
pub static GAP_APPEARE_WATCH_SPORTS                :u8=0x00C1;
pub static GAP_APPEARE_GENERIC_CLOCK               :u8=0x0100;
pub static GAP_APPEARE_GENERIC_DISPLAY             :u8=0x0140;
pub static GAP_APPEARE_GENERIC_RC                  :u8=0x0180;
pub static GAP_APPEARE_GENERIC_EYE_GALSSES         :u8=0x01C0;
pub static GAP_APPEARE_GENERIC_TAG                 :u8=0x0200;
pub static GAP_APPEARE_GENERIC_KEYRING             :u8=0x0240;
pub static GAP_APPEARE_GENERIC_MEDIA_PLAYER        :u8=0x0280;
pub static GAP_APPEARE_GENERIC_BARCODE_SCANNER     :u8=0x02C0;
pub static GAP_APPEARE_GENERIC_THERMOMETER         :u8=0x0300;
pub static GAP_APPEARE_GENERIC_THERMO_EAR          :u8=0x0301;
pub static GAP_APPEARE_GENERIC_HR_SENSOR           :u8=0x0340;
pub static GAP_APPEARE_GENERIC_HRS_BELT            :u8=0x0341;
pub static GAP_APPEARE_GENERIC_BLOOD_PRESSURE      :u8=0x0380;
pub static GAP_APPEARE_GENERIC_BP_ARM              :u8=0x0381;
pub static GAP_APPEARE_GENERIC_BP_WRIST            :u8=0x0382;
pub static GAP_APPEARE_GENERIC_HID                 :u8=0x03C0;
pub static GAP_APPEARE_HID_KEYBOARD                :u8=0x03C1;
pub static GAP_APPEARE_HID_MOUSE                   :u8=0x03C2;
pub static GAP_APPEARE_HID_JOYSTIC                 :u8=0x03C3;
pub static GAP_APPEARE_HID_GAMEPAD                 :u8=0x03C4;
pub static GAP_APPEARE_HID_DIGITIZER_TYABLET       :u8=0x03C5;
pub static GAP_APPEARE_HID_DIGITAL_CARDREADER      :u8=0x03C6;
pub static GAP_APPEARE_HID_DIGITAL_PEN             :u8=0x03C7;
pub static GAP_APPEARE_HID_BARCODE_SCANNER         :u8=0x03C8;
*/

pub static GAPROLE_PROFILEROLE: u16 = 0x300;
pub static GAPROLE_IRK: u16 = 0x301;
pub static GAPROLE_SRK: u16 = 0x302;
pub static GAPROLE_SIGNCOUNTER: u16 = 0x303;
pub static GAPROLE_BD_ADDR: u16 = 0x304;
pub static GAPROLE_ADVERT_ENABLED: u16 = 0x305;
pub static GAPROLE_ADVERT_DATA: u16 = 0x306;
pub static GAPROLE_SCAN_RSP_DATA: u16 = 0x307;
pub static GAPROLE_ADV_EVENT_TYPE: u16 = 0x308;
pub static GAPROLE_ADV_DIRECT_TYPE: u16 = 0x309;
pub static GAPROLE_ADV_DIRECT_ADDR: u16 = 0x30A;
pub static GAPROLE_ADV_CHANNEL_MAP: u16 = 0x30B;
pub static GAPROLE_ADV_FILTER_POLICY: u16 = 0x30C;
pub static GAPROLE_STATE: u16 = 0x30D;
pub static GAPROLE_MAX_SCAN_RES: u16 = 0x30E;
pub static GAPROLE_MIN_CONN_INTERVAL: u16 = 0x311;
pub static GAPROLE_MAX_CONN_INTERVAL: u16 = 0x312;

pub static GAPROLE_PHY_TX_SUPPORTED: u16 = 0x313;
pub static GAPROLE_PHY_RX_SUPPORTED: u16 = 0x314;
pub static GAPROLE_PERIODIC_ADVERT_DATA: u16 = 0x315;
pub static GAPROLE_PERIODIC_ADVERT_ENABLED: u16 = 0x316;

pub static GAPBOND_PERI_PAIRING_MODE: u16 = 0x400;
pub static GAPBOND_PERI_MITM_PROTECTION: u16 = 0x401;
pub static GAPBOND_PERI_IO_CAPABILITIES: u16 = 0x402;
pub static GAPBOND_PERI_OOB_ENABLED: u16 = 0x403;
pub static GAPBOND_PERI_OOB_DATA: u16 = 0x404;
pub static GAPBOND_PERI_BONDING_ENABLED: u16 = 0x405;
pub static GAPBOND_PERI_KEY_DIST_LIST: u16 = 0x406;
pub static GAPBOND_PERI_DEFAULT_PASSCODE: u16 = 0x407;
pub static GAPBOND_CENT_PAIRING_MODE: u16 = 0x408;
pub static GAPBOND_CENT_MITM_PROTECTION: u16 = 0x409;
pub static GAPBOND_CENT_IO_CAPABILITIES: u16 = 0x40A;
pub static GAPBOND_CENT_OOB_ENABLED: u16 = 0x40B;
pub static GAPBOND_CENT_OOB_DATA: u16 = 0x40C;
pub static GAPBOND_CENT_BONDING_ENABLED: u16 = 0x40D;
pub static GAPBOND_CENT_KEY_DIST_LIST: u16 = 0x40E;
pub static GAPBOND_CENT_DEFAULT_PASSCODE: u16 = 0x40F;
pub static GAPBOND_ERASE_ALLBONDS: u16 = 0x410;
pub static GAPBOND_AUTO_FAIL_PAIRING: u16 = 0x411;
pub static GAPBOND_AUTO_FAIL_REASON: u16 = 0x412;
pub static GAPBOND_KEYSIZE: u16 = 0x413;
pub static GAPBOND_AUTO_SYNC_WL: u16 = 0x414;
pub static GAPBOND_BOND_COUNT: u16 = 0x415;
pub static GAPBOND_BOND_FAIL_ACTION: u16 = 0x416;
pub static GAPBOND_ERASE_SINGLEBOND: u16 = 0x417;
pub static GAPBOND_BOND_AUTO: u16 = 0x418;
pub static GAPBOND_BOND_UPDATE: u16 = 0x419;
pub static GAPBOND_DISABLE_SINGLEBOND: u16 = 0x41A;
pub static GAPBOND_ENABLE_SINGLEBOND: u16 = 0x41B;
pub static GAPBOND_DISABLE_ALLBONDS: u16 = 0x41C;
pub static GAPBOND_ENABLE_ALLBONDS: u16 = 0x41D;
pub static GAPBOND_ERASE_AUTO: u16 = 0x41E;
pub static GAPBOND_AUTO_SYNC_RL: u16 = 0x41F;
pub static GAPBOND_SET_ENC_PARAMS: u16 = 0x420;
pub static GAPBOND_PERI_SC_PROTECTION: u16 = 0x421;
pub static GAPBOND_CENT_SC_PROTECTION: u16 = 0x422;

pub static GAPBOND_PAIRING_MODE_NO_PAIRING: u8 = 0x00;
pub static GAPBOND_PAIRING_MODE_WAIT_FOR_REQ: u8 = 0x01;
pub static GAPBOND_PAIRING_MODE_INITIATE: u8 = 0x02;

pub static GAPBOND_IO_CAP_DISPLAY_ONLY: u8 = 0x00;
pub static GAPBOND_IO_CAP_DISPLAY_YES_NO: u8 = 0x01;
pub static GAPBOND_IO_CAP_KEYBOARD_ONLY: u8 = 0x02;
pub static GAPBOND_IO_CAP_NO_INPUT_NO_OUTPUT: u8 = 0x03;
pub static GAPBOND_IO_CAP_KEYBOARD_DISPLAY: u8 = 0x04;

pub static GAPBOND_KEYDIST_SENCKEY: u8 = 0x01;
pub static GAPBOND_KEYDIST_SIDKEY: u8 = 0x02;
pub static GAPBOND_KEYDIST_SSIGN: u8 = 0x04;
pub static GAPBOND_KEYDIST_SLINK: u8 = 0x08;
pub static GAPBOND_KEYDIST_MENCKEY: u8 = 0x10;
pub static GAPBOND_KEYDIST_MIDKEY: u8 = 0x20;
pub static GAPBOND_KEYDIST_MSIGN: u8 = 0x40;
pub static GAPBOND_KEYDIST_MLINK: u8 = 0x80;

#[allow(non_camel_case_types, unused)]
#[derive(Clone, Copy, Debug, int_enum::IntEnum)]
#[repr(u8)]
pub enum PairingState {
    GAPBOND_PAIRING_STATE_STARTED = 0x00,
    GAPBOND_PAIRING_STATE_COMPLETE = 0x01,
    GAPBOND_PAIRING_STATE_BONDED = 0x02,
    GAPBOND_PAIRING_STATE_BOND_SAVED = 0x03,
}
/*


pub static SMP_PAIRING_FAILED_PASSKEY_ENTRY_FAILED :u8=0x01;
pub static SMP_PAIRING_FAILED_OOB_NOT_AVAIL        :u8=0x02;
pub static SMP_PAIRING_FAILED_AUTH_REQ             :u8=0x03;
pub static SMP_PAIRING_FAILED_CONFIRM_VALUE        :u8=0x04;
pub static SMP_PAIRING_FAILED_NOT_SUPPORTED        :u8=0x05;
pub static SMP_PAIRING_FAILED_ENC_KEY_SIZE         :u8=0x06;
pub static SMP_PAIRING_FAILED_CMD_NOT_SUPPORTED    :u8=0x07;
pub static SMP_PAIRING_FAILED_UNSPECIFIED          :u8=0x08;
pub static SMP_PAIRING_FAILED_REPEATED_ATTEMPTS    :u8=0x09;
pub static SMP_PAIRING_FAILED_INVALID_PARAMERERS   :u8=0x0A;
pub static SMP_PAIRING_FAILED_DHKEY_CHECK_FAILED   :u8=0x0B;
pub static SMP_PAIRING_FAILED_NUMERIC_COMPARISON   :u8=0x0C;
pub static SMP_PAIRING_FAILED_KEY_REJECTED         :u8=0x0F;


pub static GAPBOND_FAIL_NO_ACTION                  :u8=0x00;
pub static GAPBOND_FAIL_INITIATE_PAIRING           :u8=0x01;
pub static GAPBOND_FAIL_TERMINATE_LINK             :u8=0x02;
pub static GAPBOND_FAIL_TERMINATE_ERASE_BONDS      :u8=0x03;


pub static BLE_NVID_IRK                            :u8=0x02;
pub static BLE_NVID_CSRK                           :u8=0x03;
pub static BLE_NVID_SIGNCOUNTER                    :u8=0x04;


pub static BLE_NVID_BOND_RF_START                  :u8=0x10;


pub static BLE_NVID_GAP_BOND_START                 :u8=0x20;


pub static GAP_BOND_REC_ID_OFFSET                 :u8= 0;
pub static GAP_BOND_LOCAL_LTK_OFFSET              :u8= 1;
pub static GAP_BOND_DEV_LTK_OFFSET                :u8= 2;
pub static GAP_BOND_DEV_IRK_OFFSET                :u8= 3;
pub static GAP_BOND_DEV_CSRK_OFFSET               :u8= 4;
pub static GAP_BOND_DEV_SIGN_COUNTER_OFFSET       :u8= 5;
pub static GAP_BOND_REC_IDS                       :u8= 6;

*/
#[allow(non_camel_case_types, unused)]
pub enum GapRoleStates {
    GAPROLE_INIT = 0,
    GAPROLE_STARTED = 1,
    GAPROLE_ADVERTISING = 2,
    GAPROLE_WAITING = 3,
    GAPROLE_CONNECTED = 4,
    GAPROLE_CONNECTED_ADV = 5,
    GAPROLE_ERROR = 6,
}
