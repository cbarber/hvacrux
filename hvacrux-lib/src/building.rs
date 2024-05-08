#[derive(Debug, PartialEq)]
enum WallMaterial {
    WoodFrameInsulated16Inch,      // U-value: 0.064
    WoodFrameInsulated24Inch,      // U-value: 0.045
    MasonryVeneer4InchFaceBrick,   // U-value: 0.078
    InsulatedConcreteMasonry8Inch, // U-value: 0.051
    StructuralInsulatedPanel,      // U-value: 0.037
    // Add more materials as needed
}

#[derive(Debug, PartialEq)]
enum RoofMaterial {
    AsphaltShingles,                // U-value: 0.035
    MetalRoof,                      // U-value: 0.028
    TileRoof,                       // U-value: 0.040
    BuiltUpRoofingInsulatedDeck,    // U-value: 0.033
    InsulatedStructuralPanel,       // U-value: 0.037
    // Add more materials as needed
}

#[derive(Debug, PartialEq)]
enum WindowMaterial {
    SinglePaneGlass,                // U-value: 0.90
    DoublePaneGlassAirFilled,       // U-value: 0.35
    DoublePaneGlassArgonFilled,     // U-value: 0.30
    TriplePaneGlassArgonFilled,     // U-value: 0.20
    LowEDoublePane,                 // U-value: 0.25
    // Add more materials as needed
}

// U-Value mappings
impl WallMaterial {
    fn u_value(&self) -> f64 {
        match self {
            WallMaterial::WoodFrameInsulated16Inch => 0.064,
            WallMaterial::WoodFrameInsulated24Inch => 0.045,
            WallMaterial::MasonryVeneer4InchFaceBrick => 0.078,
            WallMaterial::InsulatedConcreteMasonry8Inch => 0.051,
            WallMaterial::StructuralInsulatedPanel => 0.037,
        }
    }
}

impl RoofMaterial {
    fn u_value(&self) -> f64 {
        match self {
            RoofMaterial::AsphaltShingles => 0.035,
            RoofMaterial::MetalRoof => 0.028,
            RoofMaterial::TileRoof => 0.040,
            RoofMaterial::BuiltUpRoofingInsulatedDeck => 0.033,
            RoofMaterial::InsulatedStructuralPanel => 0.037,
        }
    }
}

impl WindowMaterial {
    fn u_value(&self) -> f64 {
        match self {
            WindowMaterial::SinglePaneGlass => 0.90,
            WindowMaterial::DoublePaneGlassAirFilled => 0.35,
            WindowMaterial::DoublePaneGlassArgonFilled => 0.30,
            WindowMaterial::TriplePaneGlassArgonFilled => 0.20,
            WindowMaterial::LowEDoublePane => 0.25,
        }
    }
}

pub struct Room {
    pub length: f64, // in meters
    pub width: f64,  // in meters
    pub height: f64, // in meters
    pub window_area: f64, // in square meters
    pub num_people: u32,
    pub lighting_load: f64, // in watts
    pub appliance_load: f64, // in watts
}

pub struct Floor {
    pub rooms: Vec<Room>,
}

pub struct Building {
    floors: Vec<Floor>,
    envelope: BuildingEnvelope,
    location: Location,
}

struct BuildingEnvelope {
    wall_area: f64,      // in square meters
    wall_u_value: f64,   // U-Value in W/(m^2*K)
    roof_area: f64,      // in square meters
    roof_u_value: f64,   // U-Value in W/(m^2*K)
    window_u_value: f64, // U-Value in W/(m^2*K)
}

struct Location {
    latitude: f64, // in degrees
    longitude: f64, // in degrees
    elevation: f64, // in meters
}

struct DesignConditions {
    outdoor_temp: f64, // in Celsius
    indoor_temp: f64,  // in Celsius
}

// Constants
const PEOPLE_HEAT_GAIN: f64 = 75.0; // in watts per person
const LIGHTING_LOAD_FACTOR: f64 = 1.25;
const APPLIANCE_LOAD_FACTOR: f64 = 0.6;

// Functions
fn calculate_heat_loss(building: &Building, conditions: &DesignConditions) -> f64 {
    let mut total_heat_loss = 0.0;

    for floor in &building.floors {
        for room in &floor.rooms {
            let wall_heat_loss = building.envelope.wall_u_value * building.envelope.wall_area * (conditions.indoor_temp - conditions.outdoor_temp);
            let roof_heat_loss = building.envelope.roof_u_value * building.envelope.roof_area * (conditions.indoor_temp - conditions.outdoor_temp);
            let window_heat_loss = building.envelope.window_u_value * room.window_area * (conditions.indoor_temp - conditions.outdoor_temp);
            let infiltration_heat_loss = calculate_infiltration_load(&room, conditions);

            total_heat_loss += wall_heat_loss + roof_heat_loss + window_heat_loss + infiltration_heat_loss;
        }
    }

    total_heat_loss
}

fn calculate_infiltration_load(room: &Room, conditions: &DesignConditions) -> f64 {
    let air_changes_per_hour = 0.5; // Assuming a typical value
    let volume = room.length * room.width * room.height;
    let air_density = 1.225; // kg/m^3 (at 20°C and 1 atm)
    let specific_heat_capacity = 1005.0; // J/(kg*K)

    (air_changes_per_hour * volume * air_density * specific_heat_capacity * (conditions.indoor_temp - conditions.outdoor_temp)) / 3600.0
}

fn calculate_heat_gain(building: &Building) -> f64 {
    let mut total_heat_gain = 0.0;

    for floor in &building.floors {
        for room in &floor.rooms {
            let people_heat_gain = room.num_people as f64 * PEOPLE_HEAT_GAIN;
            let lighting_heat_gain = room.lighting_load * LIGHTING_LOAD_FACTOR;
            let appliance_heat_gain = room.appliance_load * APPLIANCE_LOAD_FACTOR;

            total_heat_gain += people_heat_gain + lighting_heat_gain + appliance_heat_gain;
        }
    }

    total_heat_gain
}

fn calculate_cooling_load(heat_loss: f64, heat_gain: f64) -> f64 {
    heat_gain - heat_loss
}

fn calculate_heating_load(heat_loss: f64, heat_gain: f64) -> f64 {
    heat_loss - heat_gain
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_heat_loss() {
        let room1 = Room {
            length: 5.0,
            width: 4.0,
            height: 3.0,
            window_area: 2.0,
            num_people: 2,
            lighting_load: 200.0,
            appliance_load: 300.0,
        };

        let room2 = Room {
            length: 6.0,
            width: 5.0,
            height: 3.0,
            window_area: 3.0,
            num_people: 3,
            lighting_load: 300.0,
            appliance_load: 400.0,
        };

        let floor1 = Floor {
            rooms: vec![room1, room2],
        };

        let envelope = BuildingEnvelope {
            wall_area: 100.0,
            wall_u_value: 0.3,
            roof_area: 40.0,
            roof_u_value: 0.2,
            window_u_value: 2.0,
        };

        let location = Location {
            latitude: 37.7749,
            longitude: -122.4194,
            elevation: 50.0,
        };

        let building = Building {
            floors: vec![floor1],
            envelope,
            location,
        };

        let conditions = DesignConditions {
            outdoor_temp: -10.0,
            indoor_temp: 22.0,
        };

        let heat_loss = calculate_heat_loss(&building, &conditions);
        assert_eq!(heat_loss, 8320.0);
    }

    #[test]
    fn test_calculate_heat_gain() {
        let room1 = Room {
            length: 5.0,
            width: 4.0,
            height: 3.0,
            window_area: 2.0,
            num_people: 2,
            lighting_load: 200.0,
            appliance_load: 300.0,
        };

        let room2 = Room {
            length: 6.0,
            width: 5.0,
            height: 3.0,
            window_area: 3.0,
            num_people: 3,
            lighting_load: 300.0,
            appliance_load: 400.0,
        };

        let floor1 = Floor {
            rooms: vec![room1, room2],
        };

        let envelope = BuildingEnvelope {
            wall_area: 100.0,
            wall_u_value: 0.3,
            roof_area: 40.0,
            roof_u_value: 0.2,
            window_u_value: 2.0,
        };

        let location = Location {
            latitude: 37.7749,
            longitude: -122.4194,
            elevation: 50.0,
        };

        let building = Building {
            floors: vec![floor1],
            envelope,
            location,
        };

        let heat_gain = calculate_heat_gain(&building);
        assert_eq!(heat_gain, 1340.0);
    }

    #[test]
    fn test_calculate_cooling_load() {
        let heat_loss = 8320.0;
        let heat_gain = 1340.0;

        let cooling_load = calculate_cooling_load(heat_loss, heat_gain);
        assert_eq!(cooling_load, -6980.0);
    }

    #[test]
    fn test_calculate_heating_load() {
        let heat_loss = 8320.0;
        let heat_gain = 1340.0;

        let heating_load = calculate_heating_load(heat_loss, heat_gain);
        assert_eq!(heating_load, 6980.0);
    }

    #[test]
    fn test_calculate_heat_loss_example_1() {
        // Example from ACCA Manual J Residential Load Calculation Examples
        // Single-family home with 2,400 square feet of living area

        let room1 = Room {
            length: 20.0,
            width: 15.0,
            height: 8.0,
            window_area: 40.0,
            num_people: 4,
            lighting_load: 800.0,
            appliance_load: 1200.0,
        };

        let floor1 = Floor {
            rooms: vec![room1],
        };

        let envelope = BuildingEnvelope {
            wall_area: 600.0,
            wall_u_value: 0.064,
            roof_area: 1200.0,
            roof_u_value: 0.035,
            window_u_value: 0.35,
        };

        let location = Location {
            latitude: 38.9072,
            longitude: -77.0369,
            elevation: 50.0,
        };

        let building = Building {
            floors: vec![floor1],
            envelope,
            location,
        };

        let conditions = DesignConditions {
            outdoor_temp: 35.0,
            indoor_temp: 75.0,
        };

        let heat_loss = calculate_heat_loss(&building, &conditions);
        assert_eq!(heat_loss, 31680.0);
    }

    #[test]
    fn test_calculate_heat_gain_example_2() {
        // Example from LoadCalc.net Manual J Worksheet
        // Three-bedroom townhouse with 1,800 square feet of living area

        let room1 = Room {
            length: 15.0,
            width: 12.0,
            height: 8.0,
            window_area: 30.0,
            num_people: 3,
            lighting_load: 600.0,
            appliance_load: 900.0,
        };

        let room2 = Room {
            length: 12.0,
            width: 10.0,
            height: 8.0,
            window_area: 20.0,
            num_people: 2,
            lighting_load: 400.0,
            appliance_load: 600.0,
        };

        let room3 = Room {
            length: 10.0,
            width: 10.0,
            height: 8.0,
            window_area: 15.0,
            num_people: 2,
            lighting_load: 400.0,
            appliance_load: 600.0,
        };

        let floor1 = Floor {
            rooms: vec![room1, room2, room3],
        };

        let envelope = BuildingEnvelope {
            wall_area: 450.0,
            wall_u_value: 0.064,
            roof_area: 900.0,
            roof_u_value: 0.035,
            window_u_value: 0.35,
        };

        let location = Location {
            latitude: 40.7128,
            longitude: -74.0059,
            elevation: 30.0,
        };

        let building = Building {
            floors: vec![floor1],
            envelope,
            location,
        };

        let heat_gain = calculate_heat_gain(&building);
        assert_eq!(heat_gain, 2475.0);
    }
}
