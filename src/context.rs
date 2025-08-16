use memflow::os::Process as MProcess;
use memflow::prelude::v1::*;

const PROCESS_NAME: &str = if cfg!(target_os = "windows") {
    "cs2.exe"
} else {
    "cs2"
};
const CLIENT_MODULE: &str = if cfg!(target_os = "windows") {
    "client.dll"
} else {
    "libclient.so"
};

const ENGINE_MODULE: &str = if cfg!(target_os = "windows") {
    "engine2.dll"
} else {
    "libengine2.so"
};

pub struct Context {
    process: IntoProcessInstanceArcBox<'static>,
    client_module: ModuleInfo,
    engine_module: ModuleInfo,
}

impl Context {
    pub fn new(os: OsInstanceArcBox<'static>) -> Self {
        let mut process = os.into_process_by_name(PROCESS_NAME).unwrap();

        let client_module = process.module_by_name(CLIENT_MODULE).unwrap();

        let engine_module = process.module_by_name(ENGINE_MODULE).unwrap();

        Context {
            process,
            client_module,
            engine_module,
        }
    }
}
