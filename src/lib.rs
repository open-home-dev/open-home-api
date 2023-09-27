// ==== Plugins ====
pub enum PluginDiscoverChannel {
    GetPluginInfo(PluginSelection),
    PluginInfo(PluginDefinition),
}

pub enum PluginSelection {
    All,
    Single(PluginId),
    Multiple(Vec<PluginId>),
}

pub struct PluginDefinition {
    id: PluginId,
    routing_key: PluginRoutingKey,
}

pub type PluginId = String;
pub type PluginRoutingKey = String;

// ==== Capability ====
//

pub enum PluginCommunicationChannel {
    GetDevices(DeviceSelection),
    Devices(Vec<DeviceDefinition>),
}

pub enum DeviceSelection {
    All,
    Single(DeviceId),
    Multiple(Vec<DeviceId>),
}

pub type DeviceId = String;

pub struct DeviceDefinition {
    id: DeviceId,
    name: String,
    capabilities: Vec<Capability>,
}

pub trait Capability {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_methods(&self) -> Vec<CapabilityMethod>;
}

pub struct CapabilityMethod {
    name: String,
    description: String,
    inputs: Vec<CapabilityIO>,
    outputs: Vec<CapabilityIO>,
}

pub struct CapabilityIO {
    title: String,
    description: Option<String>,
    capability_type: CapabilityType,
}

pub enum CapabilityType {
    String,
    Number,
    Boolean,
    Enum,
    Childs(Vec<CapabilityType>),
}

// ==== Implementations ====

pub trait OnOffCapability: Capability {
    fn set_status(&mut self, status: bool);
    fn get_status(&self) -> bool;
}

impl<T: OnOffCapability> Capability for T {
    fn get_name(&self) -> String {
        "on_off".to_string()
    }
    fn get_description(&self) -> String {
        "Turn on/off the device".to_string()
    }
    fn get_methods(&self) -> Vec<CapabilityMethod> {
        vec![
            CapabilityMethod {
                name: "set_status".to_string(),
                description: "Turn on the device".to_string(),
                inputs: vec![CapabilityIO {
                    title: "status".to_string(),
                    description: Some("The status of the device".to_string()),
                    capability_type: CapabilityType::Boolean,
                }],
                outputs: Vec::new(),
            },
            CapabilityMethod {
                name: "get_status".to_string(),
                description: "Get the status of the device".to_string(),
                inputs: Vec::new(),
                outputs: vec![CapabilityIO {
                    title: "status".to_string(),
                    description: Some("The status of the device".to_string()),
                    capability_type: CapabilityType::Boolean,
                }],
            },
        ]
    }
}

pub struct SuperLampe {
    status: bool,
}

impl OnOffCapability for SuperLampe {
    fn set_status(&mut self, status: bool) {
        self.status = status;
    }
    fn get_status(&self) -> bool {
        self.status
    }
}
