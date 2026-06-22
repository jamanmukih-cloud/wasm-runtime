pub struct WasmRuntime {
    modules: Vec<Module>,
}

struct Module { name: String }

impl WasmRuntime {
    pub fn new() -> Self { Self { modules: vec![] } }
    
    pub fn load_module(&mut self, path: &str) -> Result<&Module, Box<dyn std::error::Error>> {
        self.modules.push(Module { name: path.to_string() });
        Ok(self.modules.last().unwrap())
    }
}

impl Module {
    pub fn call(&self, _func: &str, _input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}
