use model::exception::Exception;
use model::param::Param;

#[derive(Debug)]
/// Struct containing method data from the javadoc and method declaration
pub struct Method {
    pub line_num: String,
    pub parameters: Vec<Param>,
    pub modifiers: Vec<String>,
    pub name: String,
    pub privacy: String,
    pub description: String,
    pub exceptions: Vec<Exception>,
    pub return_type: String,
}

impl Method {
    pub fn new() -> Method {
        Method {
            parameters: Vec::new(),
            modifiers: Vec::new(),
            exceptions: Vec::new(),
            line_num: String::new(),
            name: String::new(),
            privacy: String::new(),
            description: String::new(),
            return_type: String::new(),
        }
    }
    pub fn clone(&mut self) -> Method {
        let mut new_params = Vec::new();
        let mut new_excepts = Vec::new();
        let mut new_modifiers = Vec::new();

        for i in 0..self.parameters.len() {
            new_params.push(self.parameters[i].clone());
        }
        for i in 0..self.exceptions.len() {
            new_excepts.push(self.exceptions[i].clone());
        }
        for i in 0..self.modifiers.len() {
            new_modifiers.push(self.modifiers[i].clone());
        }

        Method {
            line_num: self.line_num.clone(),
            parameters: new_params,
            modifiers: new_modifiers,
            exceptions: new_excepts,
            name: self.name.clone(),
            privacy: self.privacy.clone(),
            description: self.description.clone(),
            return_type: self.return_type.clone(),
        }
    }
    pub fn clone_params(&mut self) -> Vec<Param> {
        let mut new_params = Vec::new();

        for i in 0..self.parameters.len() {
            new_params.push(self.parameters[i].clone());
        }

        new_params
    }
    pub fn ch_line_num(&mut self, value: String) {
        self.line_num = value;
    }
    pub fn ch_privacy(&mut self, value: String) {
        self.privacy = value;
    }
    pub fn add_modifier(&mut self, value: String) {
        self.modifiers.push(value);
    }
    pub fn ch_method_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn ch_description(&mut self, value: String) {
        self.description = value;
    }
    pub fn add_exception(&mut self, value: Exception) {
        self.exceptions.push(value);
    }
    pub fn add_param(&mut self, value: Param) {
        self.parameters.push(value);
    }
    pub fn ch_params(&mut self, value: Vec<Param>) {
        self.parameters = value;
    }
    pub fn ch_return_type(&mut self, value: String) {
        self.return_type = value;
    }
}
