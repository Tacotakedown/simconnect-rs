#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type CHAR = ::std::os::raw::c_char;
pub type LPCSTR = *const CHAR;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type HRESULT = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GUID {
    pub Data1: ::std::os::raw::c_ulong,
    pub Data2: ::std::os::raw::c_ushort,
    pub Data3: ::std::os::raw::c_ushort,
    pub Data4: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout__GUID() {
    const UNINIT: ::std::mem::MaybeUninit<_GUID> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_GUID>(),
        16usize,
        concat!("Size of: ", stringify!(_GUID))
    );
    assert_eq!(
        ::std::mem::align_of::<_GUID>(),
        4usize,
        concat!("Alignment of ", stringify!(_GUID))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data2) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data3) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Data4) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data4)
        )
    );
}
pub type GUID = _GUID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HWND__() {
    const UNINIT: ::std::mem::MaybeUninit<HWND__> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<HWND__>(),
        4usize,
        concat!("Size of: ", stringify!(HWND__))
    );
    assert_eq!(
        ::std::mem::align_of::<HWND__>(),
        4usize,
        concat!("Alignment of ", stringify!(HWND__))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unused) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HWND__),
            "::",
            stringify!(unused)
        )
    );
}
pub type HWND = *mut HWND__;
pub type SIMCONNECT_OBJECT_ID = DWORD;
pub const SIMCONNECT_UNUSED: DWORD = 4294967295;
pub const SIMCONNECT_OBJECT_ID_USER: DWORD = 0;
pub const SIMCONNECT_CAMERA_IGNORE_FIELD: f32 = 340282346638528860000000000000000000000.0;
pub const SIMCONNECT_CLIENTDATA_MAX_SIZE: DWORD = 8192;
pub const SIMCONNECT_GROUP_PRIORITY_HIGHEST: DWORD = 1;
pub const SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE: DWORD = 10000000;
pub const SIMCONNECT_GROUP_PRIORITY_STANDARD: DWORD = 1900000000;
pub const SIMCONNECT_GROUP_PRIORITY_DEFAULT: DWORD = 2000000000;
pub const SIMCONNECT_GROUP_PRIORITY_LOWEST: DWORD = 4000000000;
pub const MAX_METAR_LENGTH: DWORD = 2000;
pub const MAX_THERMAL_SIZE: f32 = 100000.0;
pub const MAX_THERMAL_RATE: f32 = 1000.0;
pub const INITPOSITION_AIRSPEED_CRUISE: DWORD = 4294967295;
pub const INITPOSITION_AIRSPEED_KEEP: DWORD = 4294967294;
pub const SIMCONNECT_CLIENTDATATYPE_INT8: DWORD = 4294967295;
pub const SIMCONNECT_CLIENTDATATYPE_INT16: DWORD = 4294967294;
pub const SIMCONNECT_CLIENTDATATYPE_INT32: DWORD = 4294967293;
pub const SIMCONNECT_CLIENTDATATYPE_INT64: DWORD = 4294967292;
pub const SIMCONNECT_CLIENTDATATYPE_FLOAT32: DWORD = 4294967291;
pub const SIMCONNECT_CLIENTDATATYPE_FLOAT64: DWORD = 4294967290;
pub const SIMCONNECT_CLIENTDATAOFFSET_AUTO: DWORD = 4294967295;
pub const SIMCONNECT_OPEN_CONFIGINDEX_LOCAL: DWORD = 4294967295;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL: SIMCONNECT_RECV_ID = 0;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION: SIMCONNECT_RECV_ID = 1;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN: SIMCONNECT_RECV_ID = 2;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT: SIMCONNECT_RECV_ID = 3;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT: SIMCONNECT_RECV_ID = 4;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_OBJECT_ADDREMOVE: SIMCONNECT_RECV_ID = 5;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME: SIMCONNECT_RECV_ID = 6;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME: SIMCONNECT_RECV_ID = 7;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA: SIMCONNECT_RECV_ID = 8;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE: SIMCONNECT_RECV_ID = 9;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WEATHER_OBSERVATION: SIMCONNECT_RECV_ID = 10;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLOUD_STATE: SIMCONNECT_RECV_ID = 11;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ASSIGNED_OBJECT_ID: SIMCONNECT_RECV_ID = 12;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_RESERVED_KEY: SIMCONNECT_RECV_ID = 13;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CUSTOM_ACTION: SIMCONNECT_RECV_ID = 14;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE: SIMCONNECT_RECV_ID = 15;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLIENT_DATA: SIMCONNECT_RECV_ID = 16;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_WEATHER_MODE: SIMCONNECT_RECV_ID = 17;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST: SIMCONNECT_RECV_ID = 18;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST: SIMCONNECT_RECV_ID = 19;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST: SIMCONNECT_RECV_ID = 20;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST: SIMCONNECT_RECV_ID = 21;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SERVER_STARTED:
    SIMCONNECT_RECV_ID = 22;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_CLIENT_STARTED:
    SIMCONNECT_RECV_ID = 23;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SESSION_ENDED:
    SIMCONNECT_RECV_ID = 24;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_END: SIMCONNECT_RECV_ID = 25;
pub const SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_LAP: SIMCONNECT_RECV_ID = 26;
pub type SIMCONNECT_RECV_ID = ::std::os::raw::c_int;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INVALID: SIMCONNECT_DATATYPE = 0;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT32: SIMCONNECT_DATATYPE = 1;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT64: SIMCONNECT_DATATYPE = 2;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT32: SIMCONNECT_DATATYPE = 3;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64: SIMCONNECT_DATATYPE = 4;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING8: SIMCONNECT_DATATYPE = 5;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING32: SIMCONNECT_DATATYPE = 6;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING64: SIMCONNECT_DATATYPE = 7;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING128: SIMCONNECT_DATATYPE = 8;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING256: SIMCONNECT_DATATYPE = 9;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING260: SIMCONNECT_DATATYPE = 10;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRINGV: SIMCONNECT_DATATYPE = 11;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INITPOSITION: SIMCONNECT_DATATYPE = 12;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_MARKERSTATE: SIMCONNECT_DATATYPE = 13;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_WAYPOINT: SIMCONNECT_DATATYPE = 14;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_LATLONALT: SIMCONNECT_DATATYPE = 15;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_XYZ: SIMCONNECT_DATATYPE = 16;
pub const SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_MAX: SIMCONNECT_DATATYPE = 17;
pub type SIMCONNECT_DATATYPE = ::std::os::raw::c_int;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_NONE: SIMCONNECT_EXCEPTION = 0;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_ERROR: SIMCONNECT_EXCEPTION = 1;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_SIZE_MISMATCH: SIMCONNECT_EXCEPTION = 2;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_UNRECOGNIZED_ID: SIMCONNECT_EXCEPTION = 3;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_UNOPENED: SIMCONNECT_EXCEPTION = 4;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_VERSION_MISMATCH: SIMCONNECT_EXCEPTION = 5;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_TOO_MANY_GROUPS: SIMCONNECT_EXCEPTION = 6;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_NAME_UNRECOGNIZED: SIMCONNECT_EXCEPTION = 7;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_TOO_MANY_EVENT_NAMES: SIMCONNECT_EXCEPTION = 8;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_EVENT_ID_DUPLICATE: SIMCONNECT_EXCEPTION = 9;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_TOO_MANY_MAPS: SIMCONNECT_EXCEPTION = 10;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_TOO_MANY_OBJECTS: SIMCONNECT_EXCEPTION = 11;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_TOO_MANY_REQUESTS: SIMCONNECT_EXCEPTION = 12;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_WEATHER_INVALID_PORT: SIMCONNECT_EXCEPTION = 13;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_WEATHER_INVALID_METAR: SIMCONNECT_EXCEPTION =
    14;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_WEATHER_UNABLE_TO_GET_OBSERVATION:
    SIMCONNECT_EXCEPTION = 15;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_WEATHER_UNABLE_TO_CREATE_STATION:
    SIMCONNECT_EXCEPTION = 16;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_WEATHER_UNABLE_TO_REMOVE_STATION:
    SIMCONNECT_EXCEPTION = 17;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_INVALID_DATA_TYPE: SIMCONNECT_EXCEPTION = 18;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_INVALID_DATA_SIZE: SIMCONNECT_EXCEPTION = 19;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_DATA_ERROR: SIMCONNECT_EXCEPTION = 20;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_INVALID_ARRAY: SIMCONNECT_EXCEPTION = 21;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_CREATE_OBJECT_FAILED: SIMCONNECT_EXCEPTION = 22;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_LOAD_FLIGHTPLAN_FAILED: SIMCONNECT_EXCEPTION =
    23;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OPERATION_INVALID_FOR_OBJECT_TYPE:
    SIMCONNECT_EXCEPTION = 24;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_ILLEGAL_OPERATION: SIMCONNECT_EXCEPTION = 25;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_ALREADY_SUBSCRIBED: SIMCONNECT_EXCEPTION = 26;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_INVALID_ENUM: SIMCONNECT_EXCEPTION = 27;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_DEFINITION_ERROR: SIMCONNECT_EXCEPTION = 28;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_DUPLICATE_ID: SIMCONNECT_EXCEPTION = 29;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_DATUM_ID: SIMCONNECT_EXCEPTION = 30;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OUT_OF_BOUNDS: SIMCONNECT_EXCEPTION = 31;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_ALREADY_CREATED: SIMCONNECT_EXCEPTION = 32;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OBJECT_OUTSIDE_REALITY_BUBBLE:
    SIMCONNECT_EXCEPTION = 33;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OBJECT_CONTAINER: SIMCONNECT_EXCEPTION = 34;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OBJECT_AI: SIMCONNECT_EXCEPTION = 35;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OBJECT_ATC: SIMCONNECT_EXCEPTION = 36;
pub const SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_OBJECT_SCHEDULE: SIMCONNECT_EXCEPTION = 37;
pub type SIMCONNECT_EXCEPTION = ::std::os::raw::c_int;
pub const SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_USER: SIMCONNECT_SIMOBJECT_TYPE = 0;
pub const SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_ALL: SIMCONNECT_SIMOBJECT_TYPE = 1;
pub const SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_AIRCRAFT: SIMCONNECT_SIMOBJECT_TYPE =
    2;
pub const SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_HELICOPTER:
    SIMCONNECT_SIMOBJECT_TYPE = 3;
pub const SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_BOAT: SIMCONNECT_SIMOBJECT_TYPE = 4;
pub const SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_GROUND: SIMCONNECT_SIMOBJECT_TYPE = 5;
pub type SIMCONNECT_SIMOBJECT_TYPE = ::std::os::raw::c_int;
pub const SIMCONNECT_STATE_SIMCONNECT_STATE_OFF: SIMCONNECT_STATE = 0;
pub const SIMCONNECT_STATE_SIMCONNECT_STATE_ON: SIMCONNECT_STATE = 1;
pub type SIMCONNECT_STATE = ::std::os::raw::c_int;
pub const SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_NEVER: SIMCONNECT_PERIOD = 0;
pub const SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_ONCE: SIMCONNECT_PERIOD = 1;
pub const SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_VISUAL_FRAME: SIMCONNECT_PERIOD = 2;
pub const SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME: SIMCONNECT_PERIOD = 3;
pub const SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SECOND: SIMCONNECT_PERIOD = 4;
pub type SIMCONNECT_PERIOD = ::std::os::raw::c_int;
pub const SIMCONNECT_MISSION_END_SIMCONNECT_MISSION_FAILED: SIMCONNECT_MISSION_END = 0;
pub const SIMCONNECT_MISSION_END_SIMCONNECT_MISSION_CRASHED: SIMCONNECT_MISSION_END = 1;
pub const SIMCONNECT_MISSION_END_SIMCONNECT_MISSION_SUCCEEDED: SIMCONNECT_MISSION_END = 2;
pub type SIMCONNECT_MISSION_END = ::std::os::raw::c_int;
pub const SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_NEVER:
    SIMCONNECT_CLIENT_DATA_PERIOD = 0;
pub const SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_ONCE:
    SIMCONNECT_CLIENT_DATA_PERIOD = 1;
pub const SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_VISUAL_FRAME:
    SIMCONNECT_CLIENT_DATA_PERIOD = 2;
pub const SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_ON_SET:
    SIMCONNECT_CLIENT_DATA_PERIOD = 3;
pub const SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_SECOND:
    SIMCONNECT_CLIENT_DATA_PERIOD = 4;
pub type SIMCONNECT_CLIENT_DATA_PERIOD = ::std::os::raw::c_int;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_BLACK: SIMCONNECT_TEXT_TYPE = 0;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_WHITE: SIMCONNECT_TEXT_TYPE = 1;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_RED: SIMCONNECT_TEXT_TYPE = 2;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_GREEN: SIMCONNECT_TEXT_TYPE = 3;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_BLUE: SIMCONNECT_TEXT_TYPE = 4;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_YELLOW: SIMCONNECT_TEXT_TYPE = 5;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_MAGENTA: SIMCONNECT_TEXT_TYPE = 6;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_SCROLL_CYAN: SIMCONNECT_TEXT_TYPE = 7;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_BLACK: SIMCONNECT_TEXT_TYPE = 256;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_WHITE: SIMCONNECT_TEXT_TYPE = 257;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_RED: SIMCONNECT_TEXT_TYPE = 258;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_GREEN: SIMCONNECT_TEXT_TYPE = 259;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_BLUE: SIMCONNECT_TEXT_TYPE = 260;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_YELLOW: SIMCONNECT_TEXT_TYPE = 261;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_MAGENTA: SIMCONNECT_TEXT_TYPE = 262;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_CYAN: SIMCONNECT_TEXT_TYPE = 263;
pub const SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_MENU: SIMCONNECT_TEXT_TYPE = 512;
pub type SIMCONNECT_TEXT_TYPE = ::std::os::raw::c_int;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_1: SIMCONNECT_TEXT_RESULT = 0;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_2: SIMCONNECT_TEXT_RESULT = 1;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_3: SIMCONNECT_TEXT_RESULT = 2;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_4: SIMCONNECT_TEXT_RESULT = 3;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_5: SIMCONNECT_TEXT_RESULT = 4;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_6: SIMCONNECT_TEXT_RESULT = 5;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_7: SIMCONNECT_TEXT_RESULT = 6;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_8: SIMCONNECT_TEXT_RESULT = 7;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_9: SIMCONNECT_TEXT_RESULT = 8;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_MENU_SELECT_10: SIMCONNECT_TEXT_RESULT = 9;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_DISPLAYED: SIMCONNECT_TEXT_RESULT = 65536;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_QUEUED: SIMCONNECT_TEXT_RESULT = 65537;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_REMOVED: SIMCONNECT_TEXT_RESULT = 65538;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_REPLACED: SIMCONNECT_TEXT_RESULT = 65539;
pub const SIMCONNECT_TEXT_RESULT_SIMCONNECT_TEXT_RESULT_TIMEOUT: SIMCONNECT_TEXT_RESULT = 65540;
pub type SIMCONNECT_TEXT_RESULT = ::std::os::raw::c_int;
pub const SIMCONNECT_WEATHER_MODE_SIMCONNECT_WEATHER_MODE_THEME: SIMCONNECT_WEATHER_MODE = 0;
pub const SIMCONNECT_WEATHER_MODE_SIMCONNECT_WEATHER_MODE_RWW: SIMCONNECT_WEATHER_MODE = 1;
pub const SIMCONNECT_WEATHER_MODE_SIMCONNECT_WEATHER_MODE_CUSTOM: SIMCONNECT_WEATHER_MODE = 2;
pub const SIMCONNECT_WEATHER_MODE_SIMCONNECT_WEATHER_MODE_GLOBAL: SIMCONNECT_WEATHER_MODE = 3;
pub type SIMCONNECT_WEATHER_MODE = ::std::os::raw::c_int;
pub const SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT:
    SIMCONNECT_FACILITY_LIST_TYPE = 0;
pub const SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_WAYPOINT:
    SIMCONNECT_FACILITY_LIST_TYPE = 1;
pub const SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_NDB:
    SIMCONNECT_FACILITY_LIST_TYPE = 2;
pub const SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_VOR:
    SIMCONNECT_FACILITY_LIST_TYPE = 3;
pub const SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_COUNT:
    SIMCONNECT_FACILITY_LIST_TYPE = 4;
pub type SIMCONNECT_FACILITY_LIST_TYPE = ::std::os::raw::c_int;
pub const SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL: DWORD = 1;
pub const SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER: DWORD = 2;
pub const SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE: DWORD = 4;
pub const SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME: DWORD = 8;
pub const SIMCONNECT_WAYPOINT_NONE: DWORD = 0;
pub const SIMCONNECT_WAYPOINT_SPEED_REQUESTED: DWORD = 4;
pub const SIMCONNECT_WAYPOINT_THROTTLE_REQUESTED: DWORD = 8;
pub const SIMCONNECT_WAYPOINT_COMPUTE_VERTICAL_SPEED: DWORD = 16;
pub const SIMCONNECT_WAYPOINT_ALTITUDE_IS_AGL: DWORD = 32;
pub const SIMCONNECT_WAYPOINT_ON_GROUND: DWORD = 1048576;
pub const SIMCONNECT_WAYPOINT_REVERSE: DWORD = 2097152;
pub const SIMCONNECT_WAYPOINT_WRAP_TO_FIRST: DWORD = 4194304;
pub type SIMCONNECT_EVENT_FLAG = DWORD;
pub const SIMCONNECT_EVENT_FLAG_DEFAULT: DWORD = 0;
pub const SIMCONNECT_EVENT_FLAG_FAST_REPEAT_TIMER: DWORD = 1;
pub const SIMCONNECT_EVENT_FLAG_SLOW_REPEAT_TIMER: DWORD = 2;
pub const SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY: DWORD = 16;
pub type SIMCONNECT_DATA_REQUEST_FLAG = DWORD;
pub const SIMCONNECT_DATA_REQUEST_FLAG_DEFAULT: DWORD = 0;
pub const SIMCONNECT_DATA_REQUEST_FLAG_CHANGED: DWORD = 1;
pub const SIMCONNECT_DATA_REQUEST_FLAG_TAGGED: DWORD = 2;
pub type SIMCONNECT_DATA_SET_FLAG = DWORD;
pub const SIMCONNECT_DATA_SET_FLAG_DEFAULT: DWORD = 0;
pub const SIMCONNECT_DATA_SET_FLAG_TAGGED: DWORD = 1;
pub type SIMCONNECT_CREATE_CLIENT_DATA_FLAG = DWORD;
pub const SIMCONNECT_CREATE_CLIENT_DATA_FLAG_DEFAULT: DWORD = 0;
pub const SIMCONNECT_CREATE_CLIENT_DATA_FLAG_READ_ONLY: DWORD = 1;
pub type SIMCONNECT_CLIENT_DATA_REQUEST_FLAG = DWORD;
pub const SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_DEFAULT: DWORD = 0;
pub const SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED: DWORD = 1;
pub const SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED: DWORD = 2;
pub type SIMCONNECT_CLIENT_DATA_SET_FLAG = DWORD;
pub const SIMCONNECT_CLIENT_DATA_SET_FLAG_DEFAULT: DWORD = 0;
pub const SIMCONNECT_CLIENT_DATA_SET_FLAG_TAGGED: DWORD = 1;
pub const SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_2D: DWORD = 1;
pub const SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_VIRTUAL: DWORD = 2;
pub const SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_ORTHOGONAL: DWORD = 4;
pub const SIMCONNECT_SOUND_SYSTEM_EVENT_DATA_MASTER: DWORD = 1;
pub type SIMCONNECT_NOTIFICATION_GROUP_ID = DWORD;
pub type SIMCONNECT_INPUT_GROUP_ID = DWORD;
pub type SIMCONNECT_DATA_DEFINITION_ID = DWORD;
pub type SIMCONNECT_DATA_REQUEST_ID = DWORD;
pub type SIMCONNECT_CLIENT_EVENT_ID = DWORD;
pub type SIMCONNECT_CLIENT_DATA_ID = DWORD;
pub type SIMCONNECT_CLIENT_DATA_DEFINITION_ID = DWORD;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV {
    pub dwSize: DWORD,
    pub dwVersion: DWORD,
    pub dwID: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV>(),
        12usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwSize) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV),
            "::",
            stringify!(dwSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwVersion) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV),
            "::",
            stringify!(dwVersion)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwID) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV),
            "::",
            stringify!(dwID)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EXCEPTION {
    pub _base: SIMCONNECT_RECV,
    pub dwException: DWORD,
    pub dwSendID: DWORD,
    pub dwIndex: DWORD,
}
pub const SIMCONNECT_RECV_EXCEPTION_UNKNOWN_SENDID: DWORD = 0;
pub const SIMCONNECT_RECV_EXCEPTION_UNKNOWN_INDEX: DWORD = 4294967295;
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EXCEPTION() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EXCEPTION> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EXCEPTION>(),
        24usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EXCEPTION))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EXCEPTION>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_EXCEPTION))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwException) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EXCEPTION),
            "::",
            stringify!(dwException)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwSendID) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EXCEPTION),
            "::",
            stringify!(dwSendID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwIndex) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EXCEPTION),
            "::",
            stringify!(dwIndex)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_OPEN {
    pub _base: SIMCONNECT_RECV,
    pub szApplicationName: [::std::os::raw::c_char; 256usize],
    pub dwApplicationVersionMajor: DWORD,
    pub dwApplicationVersionMinor: DWORD,
    pub dwApplicationBuildMajor: DWORD,
    pub dwApplicationBuildMinor: DWORD,
    pub dwSimConnectVersionMajor: DWORD,
    pub dwSimConnectVersionMinor: DWORD,
    pub dwSimConnectBuildMajor: DWORD,
    pub dwSimConnectBuildMinor: DWORD,
    pub dwReserved1: DWORD,
    pub dwReserved2: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_OPEN() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_OPEN> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_OPEN>(),
        308usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_OPEN))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_OPEN>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_OPEN))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szApplicationName) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(szApplicationName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwApplicationVersionMajor) as usize - ptr as usize },
        268usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwApplicationVersionMajor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwApplicationVersionMinor) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwApplicationVersionMinor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwApplicationBuildMajor) as usize - ptr as usize },
        276usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwApplicationBuildMajor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwApplicationBuildMinor) as usize - ptr as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwApplicationBuildMinor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwSimConnectVersionMajor) as usize - ptr as usize },
        284usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwSimConnectVersionMajor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwSimConnectVersionMinor) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwSimConnectVersionMinor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwSimConnectBuildMajor) as usize - ptr as usize },
        292usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwSimConnectBuildMajor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwSimConnectBuildMinor) as usize - ptr as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwSimConnectBuildMinor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwReserved1) as usize - ptr as usize },
        300usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwReserved1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwReserved2) as usize - ptr as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_OPEN),
            "::",
            stringify!(dwReserved2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_QUIT {
    pub _base: SIMCONNECT_RECV,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_QUIT() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_QUIT>(),
        12usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_QUIT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_QUIT>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_QUIT))
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT {
    pub _base: SIMCONNECT_RECV,
    pub uGroupID: DWORD,
    pub uEventID: DWORD,
    pub dwData: DWORD,
}
pub const SIMCONNECT_RECV_EVENT_UNKNOWN_GROUP: DWORD = 4294967295;
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EVENT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT>(),
        24usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EVENT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_EVENT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uGroupID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT),
            "::",
            stringify!(uGroupID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uEventID) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT),
            "::",
            stringify!(uEventID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwData) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT),
            "::",
            stringify!(dwData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_FILENAME {
    pub _base: SIMCONNECT_RECV_EVENT,
    pub szFileName: [::std::os::raw::c_char; 260usize],
    pub dwFlags: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_FILENAME() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EVENT_FILENAME> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_FILENAME>(),
        288usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EVENT_FILENAME))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_FILENAME>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_EVENT_FILENAME))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szFileName) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_FILENAME),
            "::",
            stringify!(szFileName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwFlags) as usize - ptr as usize },
        284usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_FILENAME),
            "::",
            stringify!(dwFlags)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE {
    pub _base: SIMCONNECT_RECV_EVENT,
    pub eObjType: SIMCONNECT_SIMOBJECT_TYPE,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE>(),
        28usize,
        concat!(
            "Size of: ",
            stringify!(SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).eObjType) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
            "::",
            stringify!(eObjType)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_FRAME {
    pub _base: SIMCONNECT_RECV_EVENT,
    pub fFrameRate: f32,
    pub fSimSpeed: f32,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_FRAME() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EVENT_FRAME> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_FRAME>(),
        32usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EVENT_FRAME))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_FRAME>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_EVENT_FRAME))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fFrameRate) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_FRAME),
            "::",
            stringify!(fFrameRate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fSimSpeed) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_FRAME),
            "::",
            stringify!(fSimSpeed)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED {
    pub _base: SIMCONNECT_RECV_EVENT,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED {
    pub _base: SIMCONNECT_RECV_EVENT,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED {
    pub _base: SIMCONNECT_RECV_EVENT,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_RACE_RESULT {
    pub dwNumberOfRacers: DWORD,
    pub MissionGUID: GUID,
    pub szPlayerName: [::std::os::raw::c_char; 260usize],
    pub szSessionType: [::std::os::raw::c_char; 260usize],
    pub szAircraft: [::std::os::raw::c_char; 260usize],
    pub szPlayerRole: [::std::os::raw::c_char; 260usize],
    pub fTotalTime: f64,
    pub fPenaltyTime: f64,
    pub dwIsDisqualified: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_RACE_RESULT() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_RACE_RESULT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_RACE_RESULT>(),
        1080usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_RACE_RESULT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_RACE_RESULT>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_RACE_RESULT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwNumberOfRacers) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(dwNumberOfRacers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MissionGUID) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(MissionGUID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szPlayerName) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(szPlayerName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szSessionType) as usize - ptr as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(szSessionType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szAircraft) as usize - ptr as usize },
        540usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(szAircraft)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szPlayerRole) as usize - ptr as usize },
        800usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(szPlayerRole)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fTotalTime) as usize - ptr as usize },
        1060usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(fTotalTime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fPenaltyTime) as usize - ptr as usize },
        1068usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(fPenaltyTime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwIsDisqualified) as usize - ptr as usize },
        1076usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_RACE_RESULT),
            "::",
            stringify!(dwIsDisqualified)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_RACE_END {
    pub _base: SIMCONNECT_RECV_EVENT,
    pub dwRacerNumber: DWORD,
    pub RacerData: SIMCONNECT_DATA_RACE_RESULT,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_RACE_END() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EVENT_RACE_END> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_RACE_END>(),
        1108usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EVENT_RACE_END))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_RACE_END>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_EVENT_RACE_END))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRacerNumber) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_RACE_END),
            "::",
            stringify!(dwRacerNumber)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RacerData) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_RACE_END),
            "::",
            stringify!(RacerData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_RACE_LAP {
    pub _base: SIMCONNECT_RECV_EVENT,
    pub dwLapIndex: DWORD,
    pub RacerData: SIMCONNECT_DATA_RACE_RESULT,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_RACE_LAP() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_EVENT_RACE_LAP> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_RACE_LAP>(),
        1108usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EVENT_RACE_LAP))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_RACE_LAP>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_EVENT_RACE_LAP))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwLapIndex) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_RACE_LAP),
            "::",
            stringify!(dwLapIndex)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RacerData) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_EVENT_RACE_LAP),
            "::",
            stringify!(RacerData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_SIMOBJECT_DATA {
    pub _base: SIMCONNECT_RECV,
    pub dwRequestID: DWORD,
    pub dwObjectID: DWORD,
    pub dwDefineID: DWORD,
    pub dwFlags: DWORD,
    pub dwentrynumber: DWORD,
    pub dwoutof: DWORD,
    pub dwDefineCount: DWORD,
    pub dwData: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_SIMOBJECT_DATA() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_SIMOBJECT_DATA> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_SIMOBJECT_DATA>(),
        44usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_SIMOBJECT_DATA>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRequestID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwRequestID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwObjectID) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwObjectID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwDefineID) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwDefineID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwFlags) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwFlags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwentrynumber) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwentrynumber)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwoutof) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwoutof)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwDefineCount) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwDefineCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwData) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA),
            "::",
            stringify!(dwData)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE {
    pub _base: SIMCONNECT_RECV_SIMOBJECT_DATA,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE>(),
        44usize,
        concat!(
            "Size of: ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_CLIENT_DATA {
    pub _base: SIMCONNECT_RECV_SIMOBJECT_DATA,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_CLIENT_DATA() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_CLIENT_DATA>(),
        44usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_CLIENT_DATA))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_CLIENT_DATA>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_CLIENT_DATA))
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_WEATHER_OBSERVATION {
    pub _base: SIMCONNECT_RECV,
    pub dwRequestID: DWORD,
    pub szMetar: [::std::os::raw::c_char; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_WEATHER_OBSERVATION() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_WEATHER_OBSERVATION> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_WEATHER_OBSERVATION>(),
        17usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_WEATHER_OBSERVATION))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_WEATHER_OBSERVATION>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_WEATHER_OBSERVATION)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRequestID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_WEATHER_OBSERVATION),
            "::",
            stringify!(dwRequestID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szMetar) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_WEATHER_OBSERVATION),
            "::",
            stringify!(szMetar)
        )
    );
}
pub const SIMCONNECT_CLOUD_STATE_ARRAY_WIDTH: ::std::os::raw::c_int = 64;
pub const SIMCONNECT_CLOUD_STATE_ARRAY_SIZE: ::std::os::raw::c_int = 4096;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_CLOUD_STATE {
    pub _base: SIMCONNECT_RECV,
    pub dwRequestID: DWORD,
    pub dwArraySize: DWORD,
    pub rgbData: [BYTE; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_CLOUD_STATE() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_CLOUD_STATE> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_CLOUD_STATE>(),
        21usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_CLOUD_STATE))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_CLOUD_STATE>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_CLOUD_STATE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRequestID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_CLOUD_STATE),
            "::",
            stringify!(dwRequestID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwArraySize) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_CLOUD_STATE),
            "::",
            stringify!(dwArraySize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rgbData) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_CLOUD_STATE),
            "::",
            stringify!(rgbData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_ASSIGNED_OBJECT_ID {
    pub _base: SIMCONNECT_RECV,
    pub dwRequestID: DWORD,
    pub dwObjectID: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_ASSIGNED_OBJECT_ID() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_ASSIGNED_OBJECT_ID> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_ASSIGNED_OBJECT_ID>(),
        20usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_ASSIGNED_OBJECT_ID))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_ASSIGNED_OBJECT_ID>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_ASSIGNED_OBJECT_ID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRequestID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
            "::",
            stringify!(dwRequestID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwObjectID) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
            "::",
            stringify!(dwObjectID)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_RESERVED_KEY {
    pub _base: SIMCONNECT_RECV,
    pub szChoiceReserved: [::std::os::raw::c_char; 30usize],
    pub szReservedKey: [::std::os::raw::c_char; 50usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_RESERVED_KEY() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_RESERVED_KEY> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_RESERVED_KEY>(),
        92usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_RESERVED_KEY))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_RESERVED_KEY>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_RESERVED_KEY))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szChoiceReserved) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_RESERVED_KEY),
            "::",
            stringify!(szChoiceReserved)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szReservedKey) as usize - ptr as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_RESERVED_KEY),
            "::",
            stringify!(szReservedKey)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_SYSTEM_STATE {
    pub _base: SIMCONNECT_RECV,
    pub dwRequestID: DWORD,
    pub dwInteger: DWORD,
    pub fFloat: f32,
    pub szString: [::std::os::raw::c_char; 260usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_SYSTEM_STATE() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_SYSTEM_STATE> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_SYSTEM_STATE>(),
        284usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_SYSTEM_STATE))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_SYSTEM_STATE>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_SYSTEM_STATE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRequestID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SYSTEM_STATE),
            "::",
            stringify!(dwRequestID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwInteger) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SYSTEM_STATE),
            "::",
            stringify!(dwInteger)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fFloat) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SYSTEM_STATE),
            "::",
            stringify!(fFloat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szString) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_SYSTEM_STATE),
            "::",
            stringify!(szString)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_CUSTOM_ACTION {
    pub _base: SIMCONNECT_RECV_EVENT,
    pub guidInstanceId: GUID,
    pub dwWaitForCompletion: DWORD,
    pub szPayLoad: [::std::os::raw::c_char; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_CUSTOM_ACTION() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_CUSTOM_ACTION> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_CUSTOM_ACTION>(),
        45usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_CUSTOM_ACTION))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_CUSTOM_ACTION>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_CUSTOM_ACTION))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).guidInstanceId) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_CUSTOM_ACTION),
            "::",
            stringify!(guidInstanceId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwWaitForCompletion) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_CUSTOM_ACTION),
            "::",
            stringify!(dwWaitForCompletion)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szPayLoad) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_CUSTOM_ACTION),
            "::",
            stringify!(szPayLoad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_EVENT_WEATHER_MODE {
    pub _base: SIMCONNECT_RECV_EVENT,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_EVENT_WEATHER_MODE() {
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_EVENT_WEATHER_MODE>(),
        24usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_EVENT_WEATHER_MODE))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_EVENT_WEATHER_MODE>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_RECV_EVENT_WEATHER_MODE)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_FACILITIES_LIST {
    pub _base: SIMCONNECT_RECV,
    pub dwRequestID: DWORD,
    pub dwArraySize: DWORD,
    pub dwEntryNumber: DWORD,
    pub dwOutOf: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_FACILITIES_LIST() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_FACILITIES_LIST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_FACILITIES_LIST>(),
        28usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_FACILITIES_LIST))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_FACILITIES_LIST>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_FACILITIES_LIST))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwRequestID) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_FACILITIES_LIST),
            "::",
            stringify!(dwRequestID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwArraySize) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_FACILITIES_LIST),
            "::",
            stringify!(dwArraySize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwEntryNumber) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_FACILITIES_LIST),
            "::",
            stringify!(dwEntryNumber)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwOutOf) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_FACILITIES_LIST),
            "::",
            stringify!(dwOutOf)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_FACILITY_AIRPORT {
    pub Icao: [::std::os::raw::c_char; 9usize],
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_FACILITY_AIRPORT() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_FACILITY_AIRPORT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_FACILITY_AIRPORT>(),
        33usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_FACILITY_AIRPORT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_FACILITY_AIRPORT>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_DATA_FACILITY_AIRPORT)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Icao) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_AIRPORT),
            "::",
            stringify!(Icao)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Latitude) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_AIRPORT),
            "::",
            stringify!(Latitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Longitude) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_AIRPORT),
            "::",
            stringify!(Longitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Altitude) as usize - ptr as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_AIRPORT),
            "::",
            stringify!(Altitude)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_AIRPORT_LIST {
    pub _base: SIMCONNECT_RECV_FACILITIES_LIST,
    pub rgData: [SIMCONNECT_DATA_FACILITY_AIRPORT; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_AIRPORT_LIST() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_AIRPORT_LIST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_AIRPORT_LIST>(),
        61usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_AIRPORT_LIST))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_AIRPORT_LIST>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_AIRPORT_LIST))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rgData) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_AIRPORT_LIST),
            "::",
            stringify!(rgData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_FACILITY_WAYPOINT {
    pub _base: SIMCONNECT_DATA_FACILITY_AIRPORT,
    pub fMagVar: f32,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_FACILITY_WAYPOINT() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_FACILITY_WAYPOINT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_FACILITY_WAYPOINT>(),
        37usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_FACILITY_WAYPOINT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_FACILITY_WAYPOINT>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(SIMCONNECT_DATA_FACILITY_WAYPOINT)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fMagVar) as usize - ptr as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_WAYPOINT),
            "::",
            stringify!(fMagVar)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_WAYPOINT_LIST {
    pub _base: SIMCONNECT_RECV_FACILITIES_LIST,
    pub rgData: [SIMCONNECT_DATA_FACILITY_WAYPOINT; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_WAYPOINT_LIST() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_WAYPOINT_LIST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_WAYPOINT_LIST>(),
        65usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_WAYPOINT_LIST))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_WAYPOINT_LIST>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_WAYPOINT_LIST))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rgData) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_WAYPOINT_LIST),
            "::",
            stringify!(rgData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_FACILITY_NDB {
    pub _base: SIMCONNECT_DATA_FACILITY_WAYPOINT,
    pub fFrequency: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_FACILITY_NDB() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_FACILITY_NDB> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_FACILITY_NDB>(),
        41usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_FACILITY_NDB))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_FACILITY_NDB>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_FACILITY_NDB))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fFrequency) as usize - ptr as usize },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_NDB),
            "::",
            stringify!(fFrequency)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_NDB_LIST {
    pub _base: SIMCONNECT_RECV_FACILITIES_LIST,
    pub rgData: [SIMCONNECT_DATA_FACILITY_NDB; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_NDB_LIST() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_NDB_LIST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_NDB_LIST>(),
        69usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_NDB_LIST))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_NDB_LIST>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_NDB_LIST))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rgData) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_NDB_LIST),
            "::",
            stringify!(rgData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_FACILITY_VOR {
    pub _base: SIMCONNECT_DATA_FACILITY_NDB,
    pub Flags: DWORD,
    pub fLocalizer: f32,
    pub GlideLat: f64,
    pub GlideLon: f64,
    pub GlideAlt: f64,
    pub fGlideSlopeAngle: f32,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_FACILITY_VOR() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_FACILITY_VOR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_FACILITY_VOR>(),
        77usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_FACILITY_VOR))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_FACILITY_VOR>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_FACILITY_VOR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_VOR),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fLocalizer) as usize - ptr as usize },
        45usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_VOR),
            "::",
            stringify!(fLocalizer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GlideLat) as usize - ptr as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_VOR),
            "::",
            stringify!(GlideLat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GlideLon) as usize - ptr as usize },
        57usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_VOR),
            "::",
            stringify!(GlideLon)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GlideAlt) as usize - ptr as usize },
        65usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_VOR),
            "::",
            stringify!(GlideAlt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fGlideSlopeAngle) as usize - ptr as usize },
        73usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_FACILITY_VOR),
            "::",
            stringify!(fGlideSlopeAngle)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_RECV_VOR_LIST {
    pub _base: SIMCONNECT_RECV_FACILITIES_LIST,
    pub rgData: [SIMCONNECT_DATA_FACILITY_VOR; 1usize],
}
#[test]
fn bindgen_test_layout_SIMCONNECT_RECV_VOR_LIST() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_RECV_VOR_LIST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_RECV_VOR_LIST>(),
        105usize,
        concat!("Size of: ", stringify!(SIMCONNECT_RECV_VOR_LIST))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_RECV_VOR_LIST>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_RECV_VOR_LIST))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rgData) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_RECV_VOR_LIST),
            "::",
            stringify!(rgData)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_INITPOSITION {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Pitch: f64,
    pub Bank: f64,
    pub Heading: f64,
    pub OnGround: DWORD,
    pub Airspeed: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_INITPOSITION() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_INITPOSITION> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_INITPOSITION>(),
        56usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_INITPOSITION))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_INITPOSITION>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_INITPOSITION))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Latitude) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Latitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Longitude) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Longitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Altitude) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Altitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Pitch) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Pitch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Bank) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Bank)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Heading) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Heading)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).OnGround) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(OnGround)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Airspeed) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_INITPOSITION),
            "::",
            stringify!(Airspeed)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_MARKERSTATE {
    pub szMarkerName: [::std::os::raw::c_char; 64usize],
    pub dwMarkerState: DWORD,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_MARKERSTATE() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_MARKERSTATE> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_MARKERSTATE>(),
        68usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_MARKERSTATE))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_MARKERSTATE>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_MARKERSTATE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).szMarkerName) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_MARKERSTATE),
            "::",
            stringify!(szMarkerName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwMarkerState) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_MARKERSTATE),
            "::",
            stringify!(dwMarkerState)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_WAYPOINT {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Flags: ::std::os::raw::c_ulong,
    pub ktsSpeed: f64,
    pub percentThrottle: f64,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_WAYPOINT() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_WAYPOINT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_WAYPOINT>(),
        44usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_WAYPOINT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_WAYPOINT>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_WAYPOINT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Latitude) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_WAYPOINT),
            "::",
            stringify!(Latitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Longitude) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_WAYPOINT),
            "::",
            stringify!(Longitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Altitude) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_WAYPOINT),
            "::",
            stringify!(Altitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_WAYPOINT),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ktsSpeed) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_WAYPOINT),
            "::",
            stringify!(ktsSpeed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).percentThrottle) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_WAYPOINT),
            "::",
            stringify!(percentThrottle)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_LATLONALT {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_LATLONALT() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_LATLONALT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_LATLONALT>(),
        24usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_LATLONALT))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_LATLONALT>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_LATLONALT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Latitude) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_LATLONALT),
            "::",
            stringify!(Latitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Longitude) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_LATLONALT),
            "::",
            stringify!(Longitude)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Altitude) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_LATLONALT),
            "::",
            stringify!(Altitude)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SIMCONNECT_DATA_XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[test]
fn bindgen_test_layout_SIMCONNECT_DATA_XYZ() {
    const UNINIT: ::std::mem::MaybeUninit<SIMCONNECT_DATA_XYZ> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SIMCONNECT_DATA_XYZ>(),
        24usize,
        concat!("Size of: ", stringify!(SIMCONNECT_DATA_XYZ))
    );
    assert_eq!(
        ::std::mem::align_of::<SIMCONNECT_DATA_XYZ>(),
        1usize,
        concat!("Alignment of ", stringify!(SIMCONNECT_DATA_XYZ))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_XYZ),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_XYZ),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SIMCONNECT_DATA_XYZ),
            "::",
            stringify!(z)
        )
    );
}
pub type DispatchProc = ::std::option::Option<
    unsafe extern "C" fn(
        pData: *mut SIMCONNECT_RECV,
        cbData: DWORD,
        pContext: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn SimConnect_MapClientEventToSimEvent(
        hSimConnect: HANDLE,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        EventName: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_TransmitClientEvent(
        hSimConnect: HANDLE,
        ObjectID: SIMCONNECT_OBJECT_ID,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        dwData: DWORD,
        GroupID: SIMCONNECT_NOTIFICATION_GROUP_ID,
        Flags: SIMCONNECT_EVENT_FLAG,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetSystemEventState(
        hSimConnect: HANDLE,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        dwState: SIMCONNECT_STATE,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AddClientEventToNotificationGroup(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_NOTIFICATION_GROUP_ID,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        bMaskable: BOOL,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RemoveClientEvent(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_NOTIFICATION_GROUP_ID,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetNotificationGroupPriority(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_NOTIFICATION_GROUP_ID,
        uPriority: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_ClearNotificationGroup(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_NOTIFICATION_GROUP_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestNotificationGroup(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_NOTIFICATION_GROUP_ID,
        dwReserved: DWORD,
        Flags: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AddToDataDefinition(
        hSimConnect: HANDLE,
        DefineID: SIMCONNECT_DATA_DEFINITION_ID,
        DatumName: *const ::std::os::raw::c_char,
        UnitsName: *const ::std::os::raw::c_char,
        DatumType: SIMCONNECT_DATATYPE,
        fEpsilon: f32,
        DatumID: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_ClearDataDefinition(
        hSimConnect: HANDLE,
        DefineID: SIMCONNECT_DATA_DEFINITION_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestDataOnSimObject(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        DefineID: SIMCONNECT_DATA_DEFINITION_ID,
        ObjectID: SIMCONNECT_OBJECT_ID,
        Period: SIMCONNECT_PERIOD,
        Flags: SIMCONNECT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestDataOnSimObjectType(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        DefineID: SIMCONNECT_DATA_DEFINITION_ID,
        dwRadiusMeters: DWORD,
        type_: SIMCONNECT_SIMOBJECT_TYPE,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetDataOnSimObject(
        hSimConnect: HANDLE,
        DefineID: SIMCONNECT_DATA_DEFINITION_ID,
        ObjectID: SIMCONNECT_OBJECT_ID,
        Flags: SIMCONNECT_DATA_SET_FLAG,
        ArrayCount: DWORD,
        cbUnitSize: DWORD,
        pDataSet: *mut ::std::os::raw::c_void,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_MapInputEventToClientEvent(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_INPUT_GROUP_ID,
        szInputDefinition: *const ::std::os::raw::c_char,
        DownEventID: SIMCONNECT_CLIENT_EVENT_ID,
        DownValue: DWORD,
        UpEventID: SIMCONNECT_CLIENT_EVENT_ID,
        UpValue: DWORD,
        bMaskable: BOOL,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetInputGroupPriority(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_INPUT_GROUP_ID,
        uPriority: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RemoveInputEvent(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_INPUT_GROUP_ID,
        szInputDefinition: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_ClearInputGroup(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_INPUT_GROUP_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetInputGroupState(
        hSimConnect: HANDLE,
        GroupID: SIMCONNECT_INPUT_GROUP_ID,
        dwState: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestReservedKey(
        hSimConnect: HANDLE,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        szKeyChoice1: *const ::std::os::raw::c_char,
        szKeyChoice2: *const ::std::os::raw::c_char,
        szKeyChoice3: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SubscribeToSystemEvent(
        hSimConnect: HANDLE,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        SystemEventName: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_UnsubscribeFromSystemEvent(
        hSimConnect: HANDLE,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherRequestInterpolatedObservation(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        lat: f32,
        lon: f32,
        alt: f32,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherRequestObservationAtStation(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        szICAO: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherRequestObservationAtNearestStation(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        lat: f32,
        lon: f32,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherCreateStation(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        szICAO: *const ::std::os::raw::c_char,
        szName: *const ::std::os::raw::c_char,
        lat: f32,
        lon: f32,
        alt: f32,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherRemoveStation(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        szICAO: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherSetObservation(
        hSimConnect: HANDLE,
        Seconds: DWORD,
        szMETAR: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherSetModeServer(
        hSimConnect: HANDLE,
        dwPort: DWORD,
        dwSeconds: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherSetModeTheme(
        hSimConnect: HANDLE,
        szThemeName: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherSetModeGlobal(hSimConnect: HANDLE) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherSetModeCustom(hSimConnect: HANDLE) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherSetDynamicUpdateRate(hSimConnect: HANDLE, dwRate: DWORD) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherRequestCloudState(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        minLat: f32,
        minLon: f32,
        minAlt: f32,
        maxLat: f32,
        maxLon: f32,
        maxAlt: f32,
        dwFlags: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherCreateThermal(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        lat: f32,
        lon: f32,
        alt: f32,
        radius: f32,
        height: f32,
        coreRate: f32,
        coreTurbulence: f32,
        sinkRate: f32,
        sinkTurbulence: f32,
        coreSize: f32,
        coreTransitionSize: f32,
        sinkLayerSize: f32,
        sinkTransitionSize: f32,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_WeatherRemoveThermal(
        hSimConnect: HANDLE,
        ObjectID: SIMCONNECT_OBJECT_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AICreateParkedATCAircraft(
        hSimConnect: HANDLE,
        szContainerTitle: *const ::std::os::raw::c_char,
        szTailNumber: *const ::std::os::raw::c_char,
        szAirportID: *const ::std::os::raw::c_char,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AICreateEnrouteATCAircraft(
        hSimConnect: HANDLE,
        szContainerTitle: *const ::std::os::raw::c_char,
        szTailNumber: *const ::std::os::raw::c_char,
        iFlightNumber: ::std::os::raw::c_int,
        szFlightPlanPath: *const ::std::os::raw::c_char,
        dFlightPlanPosition: f64,
        bTouchAndGo: BOOL,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AICreateNonATCAircraft(
        hSimConnect: HANDLE,
        szContainerTitle: *const ::std::os::raw::c_char,
        szTailNumber: *const ::std::os::raw::c_char,
        InitPos: SIMCONNECT_DATA_INITPOSITION,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AICreateSimulatedObject(
        hSimConnect: HANDLE,
        szContainerTitle: *const ::std::os::raw::c_char,
        InitPos: SIMCONNECT_DATA_INITPOSITION,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AIReleaseControl(
        hSimConnect: HANDLE,
        ObjectID: SIMCONNECT_OBJECT_ID,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AIRemoveObject(
        hSimConnect: HANDLE,
        ObjectID: SIMCONNECT_OBJECT_ID,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AISetAircraftFlightPlan(
        hSimConnect: HANDLE,
        ObjectID: SIMCONNECT_OBJECT_ID,
        szFlightPlanPath: *const ::std::os::raw::c_char,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_ExecuteMissionAction(hSimConnect: HANDLE, guidInstanceId: GUID) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_CompleteCustomMissionAction(
        hSimConnect: HANDLE,
        guidInstanceId: GUID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_Close(hSimConnect: HANDLE) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RetrieveString(
        pData: *mut SIMCONNECT_RECV,
        cbData: DWORD,
        pStringV: *mut ::std::os::raw::c_void,
        pszString: *mut *mut ::std::os::raw::c_char,
        pcbString: *mut DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_GetLastSentPacketID(hSimConnect: HANDLE, pdwError: *mut DWORD) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_Open(
        phSimConnect: *mut HANDLE,
        szName: LPCSTR,
        hWnd: HWND,
        UserEventWin32: DWORD,
        hEventHandle: HANDLE,
        ConfigIndex: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_CallDispatch(
        hSimConnect: HANDLE,
        pfcnDispatch: DispatchProc,
        pContext: *mut ::std::os::raw::c_void,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_GetNextDispatch(
        hSimConnect: HANDLE,
        ppData: *mut *mut SIMCONNECT_RECV,
        pcbData: *mut DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestResponseTimes(
        hSimConnect: HANDLE,
        nCount: DWORD,
        fElapsedSeconds: *mut f32,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_InsertString(
        pDest: *mut ::std::os::raw::c_char,
        cbDest: DWORD,
        ppEnd: *mut *mut ::std::os::raw::c_void,
        pcbStringV: *mut DWORD,
        pSource: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_CameraSetRelative6DOF(
        hSimConnect: HANDLE,
        fDeltaX: f32,
        fDeltaY: f32,
        fDeltaZ: f32,
        fPitchDeg: f32,
        fBankDeg: f32,
        fHeadingDeg: f32,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_MenuAddItem(
        hSimConnect: HANDLE,
        szMenuItem: *const ::std::os::raw::c_char,
        MenuEventID: SIMCONNECT_CLIENT_EVENT_ID,
        dwData: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_MenuDeleteItem(
        hSimConnect: HANDLE,
        MenuEventID: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_MenuAddSubItem(
        hSimConnect: HANDLE,
        MenuEventID: SIMCONNECT_CLIENT_EVENT_ID,
        szMenuItem: *const ::std::os::raw::c_char,
        SubMenuEventID: SIMCONNECT_CLIENT_EVENT_ID,
        dwData: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_MenuDeleteSubItem(
        hSimConnect: HANDLE,
        MenuEventID: SIMCONNECT_CLIENT_EVENT_ID,
        SubMenuEventID: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestSystemState(
        hSimConnect: HANDLE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        szState: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetSystemState(
        hSimConnect: HANDLE,
        szState: *const ::std::os::raw::c_char,
        dwInteger: DWORD,
        fFloat: f32,
        szString: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_MapClientDataNameToID(
        hSimConnect: HANDLE,
        szClientDataName: *const ::std::os::raw::c_char,
        ClientDataID: SIMCONNECT_CLIENT_DATA_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_CreateClientData(
        hSimConnect: HANDLE,
        ClientDataID: SIMCONNECT_CLIENT_DATA_ID,
        dwSize: DWORD,
        Flags: SIMCONNECT_CREATE_CLIENT_DATA_FLAG,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_AddToClientDataDefinition(
        hSimConnect: HANDLE,
        DefineID: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        dwOffset: DWORD,
        dwSizeOrType: DWORD,
        fEpsilon: f32,
        DatumID: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_ClearClientDataDefinition(
        hSimConnect: HANDLE,
        DefineID: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestClientData(
        hSimConnect: HANDLE,
        ClientDataID: SIMCONNECT_CLIENT_DATA_ID,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
        DefineID: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        Period: SIMCONNECT_CLIENT_DATA_PERIOD,
        Flags: SIMCONNECT_CLIENT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SetClientData(
        hSimConnect: HANDLE,
        ClientDataID: SIMCONNECT_CLIENT_DATA_ID,
        DefineID: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        Flags: SIMCONNECT_CLIENT_DATA_SET_FLAG,
        dwReserved: DWORD,
        cbUnitSize: DWORD,
        pDataSet: *mut ::std::os::raw::c_void,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_FlightLoad(
        hSimConnect: HANDLE,
        szFileName: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_FlightSave(
        hSimConnect: HANDLE,
        szFileName: *const ::std::os::raw::c_char,
        szTitle: *const ::std::os::raw::c_char,
        szDescription: *const ::std::os::raw::c_char,
        Flags: DWORD,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_FlightPlanLoad(
        hSimConnect: HANDLE,
        szFileName: *const ::std::os::raw::c_char,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_Text(
        hSimConnect: HANDLE,
        type_: SIMCONNECT_TEXT_TYPE,
        fTimeSeconds: f32,
        EventID: SIMCONNECT_CLIENT_EVENT_ID,
        cbUnitSize: DWORD,
        pDataSet: *mut ::std::os::raw::c_void,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_SubscribeToFacilities(
        hSimConnect: HANDLE,
        type_: SIMCONNECT_FACILITY_LIST_TYPE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_UnsubscribeToFacilities(
        hSimConnect: HANDLE,
        type_: SIMCONNECT_FACILITY_LIST_TYPE,
    ) -> HRESULT;
}
extern "C" {
    pub fn SimConnect_RequestFacilitiesList(
        hSimConnect: HANDLE,
        type_: SIMCONNECT_FACILITY_LIST_TYPE,
        RequestID: SIMCONNECT_DATA_REQUEST_ID,
    ) -> HRESULT;
}

use std::ffi::CString;
use std::mem::transmute_copy;
use std::ptr;

/// Enumerations for all the possible data types received from SimConnect
#[derive(Debug)]
pub enum DispatchResult<'a> {
    Null,
    Exception(&'a SIMCONNECT_RECV_EXCEPTION),
    Open(&'a SIMCONNECT_RECV_OPEN),
    Quit(&'a SIMCONNECT_RECV_QUIT),
    Event(&'a SIMCONNECT_RECV_EVENT),
    EventObjectAddRemove(&'a SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
    EventFilename(&'a SIMCONNECT_RECV_EVENT_FILENAME),
    EventFrame(&'a SIMCONNECT_RECV_EVENT_FRAME),
    SimObjectData(&'a SIMCONNECT_RECV_SIMOBJECT_DATA),
    SimObjectDataByType(&'a SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
    WeatherObservation(&'a SIMCONNECT_RECV_WEATHER_OBSERVATION),
    CloudState(&'a SIMCONNECT_RECV_CLOUD_STATE),
    AssignedObjectId(&'a SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
    ReservedKey(&'a SIMCONNECT_RECV_RESERVED_KEY),
    CustomAction(&'a SIMCONNECT_RECV_CUSTOM_ACTION),
    SystemState(&'a SIMCONNECT_RECV_SYSTEM_STATE),
    ClientData(&'a SIMCONNECT_RECV_CLIENT_DATA),
    EventWeatherMode(&'a SIMCONNECT_RECV_EVENT_WEATHER_MODE),
    AirportList(&'a SIMCONNECT_RECV_AIRPORT_LIST),
    VorList(&'a SIMCONNECT_RECV_VOR_LIST),
    NdbList(&'a SIMCONNECT_RECV_NDB_LIST),
    WaypointList(&'a SIMCONNECT_RECV_WAYPOINT_LIST),
    EventMultiplayerServerStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
    EventMultiplayerClientStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
    EventMultiplayerSessionEnded(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
    EventRaceEnd(&'a SIMCONNECT_RECV_EVENT_RACE_END),
    EventRaceLap(&'a SIMCONNECT_RECV_EVENT_RACE_LAP),
}

/// Handles communication between the client program and SimConnect
/// For more information about the functions provided, refer to the SimConnect SDK Documentation. The functions name closely match up with those defined there.
#[derive(Debug)]
pub struct SimConnector {
    sim_connect_handle: HANDLE,
}

impl Default for SimConnector {
    fn default() -> Self {
        Self {
            sim_connect_handle: std::ptr::null_mut(),
        }
    }
}

impl SimConnector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, program_name: &str) -> bool {
        unsafe {
            let temp_1 = ptr::null_mut();
            let temp_2 = ptr::null_mut();

            let program_name = CString::new(program_name).unwrap();

            SimConnect_Open(
                &mut self.sim_connect_handle,
                program_name.as_ptr(),
                temp_1,
                0,
                temp_2,
                0,
            );

            !self.sim_connect_handle.is_null()
        }
    }

    pub fn add_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        datum_name: &str,
        units_name: &str,
        datum_type: SIMCONNECT_DATATYPE,
        datum_id: DWORD,
        epsilon: f32,
    ) -> bool {
        let datum_name = CString::new(datum_name).unwrap();
        let units_name = CString::new(units_name).unwrap();

        unsafe {
            SimConnect_AddToDataDefinition(
                self.sim_connect_handle,
                define_id,
                datum_name.as_ptr(),
                units_name.as_ptr(),
                datum_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn set_system_event_state(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        state: SIMCONNECT_STATE,
    ) -> bool {
        unsafe { SimConnect_SetSystemEventState(self.sim_connect_handle, event_id, state) == 0 }
    }

    pub fn remove_client_event(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe { SimConnect_RemoveClientEvent(self.sim_connect_handle, group_id, event_id) == 0 }
    }

    pub fn clear_notification_group(&self, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearNotificationGroup(self.sim_connect_handle, group_id) == 0 }
    }

    pub fn request_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        reserved: DWORD,
        flags: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestNotificationGroup(self.sim_connect_handle, group_id, reserved, flags)
                == 0
        }
    }

    pub fn clear_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearDataDefinition(self.sim_connect_handle, define_id) == 0 }
    }

    pub fn create_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        size: DWORD,
        flags: SIMCONNECT_CREATE_CLIENT_DATA_FLAG,
    ) -> bool {
        unsafe { SimConnect_CreateClientData(self.sim_connect_handle, data_id, size, flags) == 0 }
    }

    pub fn request_data_on_sim_object_type(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        radius_in_meters: DWORD,
        object_type: SIMCONNECT_SIMOBJECT_TYPE,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObjectType(
                self.sim_connect_handle,
                request_id,
                define_id,
                radius_in_meters,
                object_type,
            ) == 0
        }
    }

    pub fn remove_input_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
    ) -> bool {
        let input_definition = CString::new(input_definition).unwrap();

        unsafe {
            SimConnect_RemoveInputEvent(
                self.sim_connect_handle,
                group_id,
                input_definition.as_ptr(),
            ) == 0
        }
    }

    pub fn clear_input_group(&self, group_id: SIMCONNECT_INPUT_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearInputGroup(self.sim_connect_handle, group_id) == 0 }
    }

    pub fn request_reserved_Key(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        key_choice_1: &str,
        key_choice_2: &str,
        key_choice_3: &str,
    ) -> bool {
        let key_choice_1 = CString::new(key_choice_1).unwrap();
        let key_choice_2 = CString::new(key_choice_2).unwrap();
        let key_choice_3 = CString::new(key_choice_3).unwrap();

        unsafe {
            SimConnect_RequestReservedKey(
                self.sim_connect_handle,
                event_id,
                key_choice_1.as_ptr(),
                key_choice_2.as_ptr(),
                key_choice_3.as_ptr(),
            ) == 0
        }
    }

    pub fn unsubscribe_from_system_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_UnsubscribeFromSystemEvent(self.sim_connect_handle, event_id) == 0 }
    }

    pub fn ai_create_parked_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        airport_id: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let container_title = CString::new(container_title).unwrap();
        let tail_number = CString::new(tail_number).unwrap();
        let airport_id = CString::new(airport_id).unwrap();

        unsafe {
            SimConnect_AICreateParkedATCAircraft(
                self.sim_connect_handle,
                container_title.as_ptr(),
                tail_number.as_ptr(),
                airport_id.as_ptr(),
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_enroute_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        flight_number: i32,
        flight_plan_path: &str,
        flight_plan_position: f64,
        touch_and_go: bool,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let container_title = CString::new(container_title).unwrap();
        let tail_number = CString::new(tail_number).unwrap();
        let flight_plan_path = CString::new(flight_plan_path).unwrap();

        unsafe {
            SimConnect_AICreateEnrouteATCAircraft(
                self.sim_connect_handle,
                container_title.as_ptr(),
                tail_number.as_ptr(),
                flight_number,
                flight_plan_path.as_ptr(),
                flight_plan_position,
                touch_and_go as i32,
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_non_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let container_title = CString::new(container_title).unwrap();
        let tail_number = CString::new(tail_number).unwrap();

        unsafe {
            SimConnect_AICreateNonATCAircraft(
                self.sim_connect_handle,
                container_title.as_ptr(),
                tail_number.as_ptr(),
                init_pos,
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_simulated_object(
        &self,
        container_title: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let container_title = CString::new(container_title).unwrap();

        unsafe {
            SimConnect_AICreateSimulatedObject(
                self.sim_connect_handle,
                container_title.as_ptr(),
                init_pos,
                request_id,
            ) == 0
        }
    }

    pub fn ai_release_control(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIReleaseControl(self.sim_connect_handle, object_id, request_id) == 0 }
    }

    pub fn ai_remove_object(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIRemoveObject(self.sim_connect_handle, object_id, request_id) == 0 }
    }

    pub fn ai_set_aircraft_flight_plan(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        flight_plan_path: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        let flight_plan_path = CString::new(flight_plan_path).unwrap();

        unsafe {
            SimConnect_AISetAircraftFlightPlan(
                self.sim_connect_handle,
                object_id,
                flight_plan_path.as_ptr(),
                request_id,
            ) == 0
        }
    }

    pub fn execute_mission_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_ExecuteMissionAction(self.sim_connect_handle, instance_id) == 0 }
    }

    pub fn complete_custom_mission_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_CompleteCustomMissionAction(self.sim_connect_handle, instance_id) == 0 }
    }

    pub fn close(&self) -> bool {
        unsafe { SimConnect_Close(self.sim_connect_handle) == 0 }
    }

    pub unsafe fn get_last_sent_packet_id(&self, error: *mut DWORD) -> bool {
        unsafe { SimConnect_GetLastSentPacketID(self.sim_connect_handle, error) == 0 }
    }

    // not tested
    pub unsafe fn call_dispatch(
        &self,
        dispatch_callback: DispatchProc,
        context: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe { SimConnect_CallDispatch(self.sim_connect_handle, dispatch_callback, context) == 0 }
    }

    pub unsafe fn request_response_times(&self, count: DWORD, elapsed_seconds: *mut f32) -> bool {
        unsafe {
            SimConnect_RequestResponseTimes(self.sim_connect_handle, count, elapsed_seconds) == 0
        }
    }

    pub fn camera_set_relative_6dof(
        &self,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        pitch: f32,
        bank: f32,
        heading: f32,
    ) -> bool {
        unsafe {
            SimConnect_CameraSetRelative6DOF(
                self.sim_connect_handle,
                delta_x,
                delta_y,
                delta_z,
                pitch,
                bank,
                heading,
            ) == 0
        }
    }

    pub fn menu_add_item(
        &self,
        menu_item: &str,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> bool {
        let menu_item = CString::new(menu_item).unwrap();

        unsafe {
            SimConnect_MenuAddItem(self.sim_connect_handle, menu_item.as_ptr(), event_id, data) == 0
        }
    }

    pub fn menu_delete_item(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_MenuDeleteItem(self.sim_connect_handle, event_id) == 0 }
    }

    pub fn menu_delete_sub_item(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        sub_event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe {
            SimConnect_MenuDeleteSubItem(self.sim_connect_handle, event_id, sub_event_id) == 0
        }
    }

    pub fn request_system_state(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        state: &str,
    ) -> bool {
        let state = CString::new(state).unwrap();

        unsafe {
            SimConnect_RequestSystemState(self.sim_connect_handle, request_id, state.as_ptr()) == 0
        }
    }

    pub fn map_client_data_name_to_id(
        &self,
        client_data_name: &str,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
    ) -> bool {
        let client_data_name = CString::new(client_data_name).unwrap();

        unsafe {
            SimConnect_MapClientDataNameToID(
                self.sim_connect_handle,
                client_data_name.as_ptr(),
                data_id,
            ) == 0
        }
    }

    pub fn add_to_client_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        offset: DWORD,
        size_or_type: DWORD,
        epsilon: f32,
        datum_id: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_AddToClientDataDefinition(
                self.sim_connect_handle,
                define_id,
                offset,
                size_or_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn clear_client_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearClientDataDefinition(self.sim_connect_handle, define_id) == 0 }
    }

    pub fn request_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_CLIENT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestClientData(
                self.sim_connect_handle,
                data_id,
                request_id,
                define_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub unsafe fn set_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        flags: DWORD,
        reserved: DWORD,
        unit_size: DWORD,
        data_set: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetClientData(
                self.sim_connect_handle,
                data_id,
                define_id,
                flags,
                reserved,
                unit_size,
                data_set,
            ) == 0
        }
    }

    pub fn flight_load(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { SimConnect_FlightLoad(self.sim_connect_handle, file_name.as_ptr()) == 0 }
    }

    pub unsafe fn text(
        &self,
        text_type: SIMCONNECT_TEXT_TYPE,
        time_in_seconds: f32,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        unit_size: DWORD,
        data_set: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_Text(
                self.sim_connect_handle,
                text_type,
                time_in_seconds,
                event_id,
                unit_size,
                data_set,
            ) == 0
        }
    }

    pub unsafe fn subscribe_to_facilities(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_SubscribeToFacilities(self.sim_connect_handle, list_type, request_id) == 0
        }
    }

    pub fn unsubscribe_to_facilities(&self, list_type: SIMCONNECT_FACILITY_LIST_TYPE) -> bool {
        unsafe { SimConnect_UnsubscribeToFacilities(self.sim_connect_handle, list_type) == 0 }
    }

    pub fn request_facilities_list(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_RequestFacilitiesList(self.sim_connect_handle, list_type, request_id) == 0
        }
    }

    pub fn request_data_on_sim_object(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObject(
                self.sim_connect_handle,
                request_id,
                define_id,
                object_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub unsafe fn set_data_on_sim_object(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        flags: SIMCONNECT_DATA_SET_FLAG,
        array_count: DWORD,
        size: DWORD,
        pntr: *mut ::std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetDataOnSimObject(
                self.sim_connect_handle,
                define_id,
                object_id,
                flags,
                array_count,
                size,
                pntr,
            ) == 0
        }
    }

    pub fn subscribe_to_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        let event_name = CString::new(event_name).unwrap();

        unsafe {
            SimConnect_SubscribeToSystemEvent(
                self.sim_connect_handle,
                event_id,
                event_name.as_ptr(),
            ) == 0
        }
    }

    pub fn map_client_event_to_sim_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        let event_name = CString::new(event_name).unwrap();

        unsafe {
            SimConnect_MapClientEventToSimEvent(
                self.sim_connect_handle,
                event_id,
                event_name.as_ptr(),
            ) == 0
        }
    }

    pub fn transmit_client_event(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        dw_data: DWORD,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: SIMCONNECT_EVENT_FLAG,
    ) -> bool {
        unsafe {
            SimConnect_TransmitClientEvent(
                self.sim_connect_handle,
                object_id,
                event_id,
                dw_data,
                group_id,
                flags,
            ) == 0
        }
    }

    pub fn add_client_event_to_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        maskable: bool,
    ) -> bool {
        unsafe {
            SimConnect_AddClientEventToNotificationGroup(
                self.sim_connect_handle,
                group_id,
                event_id,
                maskable as i32,
            ) == 0
        }
    }

    pub fn set_notification_group_priority(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        priority: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_SetNotificationGroupPriority(self.sim_connect_handle, group_id, priority)
                == 0
        }
    }

    pub fn map_input_event_to_client_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
        down_event: SIMCONNECT_CLIENT_EVENT_ID,
        down_return_value: DWORD,
        up_event: SIMCONNECT_CLIENT_EVENT_ID,
        up_return_value: DWORD,
        maskable: bool,
    ) -> bool {
        let input_definition = CString::new(input_definition).unwrap();

        unsafe {
            SimConnect_MapInputEventToClientEvent(
                self.sim_connect_handle,
                group_id,
                input_definition.as_ptr(),
                down_event,
                down_return_value,
                up_event,
                up_return_value,
                maskable as i32,
            ) == 0
        }
    }

    pub fn set_input_group_state(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, state: DWORD) -> bool {
        unsafe { SimConnect_SetInputGroupState(self.sim_connect_handle, group_id, state) == 0 }
    }

    pub fn set_input_priority(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, priority: DWORD) -> bool {
        unsafe {
            SimConnect_SetInputGroupPriority(self.sim_connect_handle, group_id, priority) == 0
        }
    }

    /// Retrieves the next message from SimConnect. Nonblocking.
    pub fn get_next_message(&self) -> Result<DispatchResult, &str> {
        let mut data_buf: *mut SIMCONNECT_RECV = ptr::null_mut();

        let mut size_buf: DWORD = 32;
        let size_buf_pointer: *mut DWORD = &mut size_buf;

        unsafe {
            let result = SimConnect_GetNextDispatch(
                self.sim_connect_handle,
                &mut data_buf,
                size_buf_pointer,
            );
            if result != 0 {
                return Err("Failed getting data!");
            }

            return match (*data_buf).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Ok(DispatchResult::Null),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => Ok(DispatchResult::Exception(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_EXCEPTION)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Ok(DispatchResult::Open(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_OPEN)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Ok(DispatchResult::Quit(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_QUIT)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => Ok(DispatchResult::Event(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_EVENT)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_OBJECT_ADDREMOVE => {
                    Ok(DispatchResult::EventObjectAddRemove(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME => {
                    Ok(DispatchResult::EventFilename(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_FILENAME),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME => {
                    Ok(DispatchResult::EventFrame(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_FRAME),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    Ok(DispatchResult::SimObjectData(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => {
                    Ok(DispatchResult::SimObjectDataByType(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WEATHER_OBSERVATION => {
                    Ok(DispatchResult::WeatherObservation(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_WEATHER_OBSERVATION),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLOUD_STATE => {
                    Ok(DispatchResult::CloudState(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CLOUD_STATE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ASSIGNED_OBJECT_ID => {
                    Ok(DispatchResult::AssignedObjectId(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_RESERVED_KEY => {
                    Ok(DispatchResult::ReservedKey(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_RESERVED_KEY),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CUSTOM_ACTION => {
                    Ok(DispatchResult::CustomAction(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CUSTOM_ACTION),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE => {
                    Ok(DispatchResult::SystemState(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SYSTEM_STATE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLIENT_DATA => {
                    Ok(DispatchResult::ClientData(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CLIENT_DATA),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_WEATHER_MODE => {
                    Ok(DispatchResult::EventWeatherMode(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_WEATHER_MODE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                    Ok(DispatchResult::AirportList(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_AIRPORT_LIST),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST => Ok(DispatchResult::VorList(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_VOR_LIST)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST => Ok(DispatchResult::NdbList(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_NDB_LIST)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST => {
                    Ok(DispatchResult::WaypointList(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_WAYPOINT_LIST),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SERVER_STARTED => Ok(
                    DispatchResult::EventMultiplayerServerStarted(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_CLIENT_STARTED => Ok(
                    DispatchResult::EventMultiplayerClientStarted(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SESSION_ENDED => Ok(
                    DispatchResult::EventMultiplayerSessionEnded(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_END => {
                    Ok(DispatchResult::EventRaceEnd(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_END),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_LAP => {
                    Ok(DispatchResult::EventRaceLap(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_LAP),
                    )))
                }

                _ => Err("Unhandled RECV_ID"),
            };
        }
    }
}

impl Drop for SimConnector {
    fn drop(&mut self) {
        if !self.sim_connect_handle.is_null() {
            self.close();
        }
    }
}
