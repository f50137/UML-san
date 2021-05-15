#[derive(Debug)]
pub struct Type(pub String);

#[derive(Debug, PartialEq, Eq)]
pub enum Capsulation {
    Public,
    Private,
    Protected,
}

impl Capsulation {
    fn to_char(&self) -> char {
        match *self {
            Self::Public => '+',
            Self::Private => '-',
            Self::Protected => '#'
        }
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
    pub capsulation: Capsulation,
}

impl Field {
    pub fn new(field_type: Type, name: String, capsulation: Capsulation) -> Self {
        Self {
            name, field_type, capsulation
        }
    }

    pub fn to_string(&self) -> String {
        let Type(type_name) = &self.field_type;
        format!("{} {}: {}", 
                self.capsulation.to_char(),
                self.name,
                type_name)
    }
}

#[derive(Debug)]
pub struct Parameter(pub Type, pub String);

impl Parameter {
    fn to_string(&self) -> String {
        let Parameter(Type(type_name), name) = self;
        format!("{}: {}", name, type_name)
    }
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
    pub capsulation: Capsulation,
}

impl Method {
    pub fn is_constructor(&self) -> bool {
        let Type(type_name) = &self.return_type;
        type_name.len() == 0
    }

    pub fn new(name: String, return_type: Type, parameters: Vec<Parameter>, capsulation: Capsulation) -> Self {
        Self {
            name, return_type, parameters, capsulation,
        }
    }

    pub fn new_constructor(name: String, parameters: Vec<Parameter>, capsulation: Capsulation) -> Self {
        Self {
            name, return_type: Type("".to_string()), parameters, capsulation,
        }
    }

    pub fn to_string(&self) -> String {
        let Type(return_type_name) = &self.return_type;
        let mut out = format!("{} {}(", self.capsulation.to_char(), self.name);
       
        if self.parameters.len() > 0 {
            for i in 0..self.parameters.len() - 1 {
                out.push_str(&format!("{}, ", self.parameters[i].to_string()));
            }

            out.push_str(&format!("{})", self.parameters.last().unwrap().to_string()));
            if !self.is_constructor() {
                out.push_str(&format!(": {}", return_type_name))
            }
        } else {
            if self.is_constructor() {
                out.push(')');
            } else {
                out.push_str(&format!("): {}", return_type_name));
            }
        }

        out
    }
}

pub enum Declaration {
    Method(Method),
    Field(Field)
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}

impl Class {
    pub fn new(name: String, fields: Vec<Field>, methods: Vec<Method>) -> Self {
        Self {
            name, fields, methods
        }
    }
}
