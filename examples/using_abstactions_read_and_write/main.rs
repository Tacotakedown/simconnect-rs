use simconnect::{
    get_simvar_data, register_simvars, DispatchResult, SimConnector, SimVarRegistration,
    SimVarStruct,
};

struct LonLatSimVarStruct {
    lat: f64,
    lon: f64,
}

impl SimVarStruct for LonLatSimVarStruct {
    fn from_raw(data: u32) -> Self {
        unsafe { std::ptr::read_unaligned(data as *const LonLatSimVarStruct) }
    }
}

enum RegistrationEnum {
    LAT_LON,
}

fn main() {
    let mut connector = SimConnector::new();
    connector.connect("Batch simvars");
    let simvars = vec![
        SimVarRegistration {
            name: "PLANE LATTITUDE",
            unit: "Degrees",
        },
        SimVarRegistration {
            name: "PLANE LONGITIUDE",
            unit: "Degrees",
        },
    ]; // note the order we are defining them in, its the same as the struct

    register_simvars(&mut connector, simvars, RegistrationEnum::LAT_LON as u32);

    connector.request_data_on_sim_object(
        0,
        0,
        0,
        simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
        0,
        0,
        0,
        0,
    );

    loop {
        match connector.get_next_message() {
            Ok(DispatchResult::SimObjectData(data)) => {
                if data.dwDefineID == RegistrationEnum::LAT_LON as u32 {
                    let sim_data_value: LonLatSimVarStruct = get_simvar_data(&data);
                    println!(
                        "Latitude: {}, Longitude: {} ",
                        sim_data_value.lat, sim_data_value.lon
                    );
                }
            }
            Ok(_) => {
                // handle all the other events
            }
            Err(e) => {
                println!("Error reading message: {}", e);
            }
        }
    }
}
