use std::collections::HashMap;
use std::io::Result;
use std::io;
use std::io::Stdout;
use std::io::Write;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Comparison {
    pub fn from_str(s: &str) -> Self {
        match s {
            "==" => Comparison::Equal,
            "!=" => Comparison::NotEqual,
            "<" => Comparison::LessThan,
            "<=" => Comparison::LessThanOrEqual,
            ">" => Comparison::GreaterThan,
            ">=" => Comparison::GreaterThanOrEqual,
            _ => panic!("Invalid comparison operator: {}", s),
        }
    }
}

pub struct Function {
    parameters: Vec<String>,
    code: Vec<String>,
}

pub struct Interpreter<T: Write> {
    variables: HashMap<String, i32>,
    arrays: HashMap<String, Vec<i32>>,
    strings: HashMap<String, String>,
    float: HashMap<String, f32>,
    functions: HashMap<String, Function>,
    structs: HashMap<String, HashMap<String, i32>>,
    output_stream: T
}

impl Interpreter<Stdout> {
    pub fn new() -> Interpreter<io::Stdout> {
        Interpreter {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            strings: HashMap::new(),
            float: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            output_stream: io::stdout()
        }
    }
}

impl<T: Write> Interpreter<T> {
    pub fn new_with_output_stream(output_stream: T) -> Self {
        Interpreter {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            strings: HashMap::new(),
            float: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            output_stream: output_stream
        }
    }

    pub fn run(&mut self, source_code: &str) -> Result<&T> {
        let mut source = source_code.split_whitespace();
        while let Some(word) = source.next() {
            match word {
                "var" => {
                    let name = source.next().unwrap();
                    let value = source.next().unwrap().parse().unwrap();
                    self.variables.insert(name.to_owned(), value);
                }
                "array" => {
                    let name = source.next().unwrap();
                    let size = source.next().unwrap().parse().unwrap();
                    let mut array = Vec::with_capacity(size);
                    for _ in 0..size {
                        let value = source.next().unwrap().parse().unwrap();
                        array.push(value);
                    }
                    self.arrays.insert(name.to_owned(), array);
                }
                "string" => {
                    let name = source.next().unwrap();
                    let mut value = String::new();
                    while let Some(word) = source.next() {
                        if word == "endstring" {
                            break;
                        }
                        value.push_str(word);
                        value.push(' ');
                    }
                    value.pop();
                    self.strings.insert(name.to_owned(), value);
                }
                "float" => {
                    let name = source.next().unwrap();
                    let value = source.next().unwrap().parse().unwrap();
                    self.float.insert(name.to_owned(), value);
                }
                "function" => {
                    let name = source.next().unwrap();
                    let mut function = vec![];
                    let mut parameters = vec![];
                    while let Some(word) = source.next() {
                        if word == "end" {
                            break;
                        }
                        if word == "with" {
                            while let Some(word) = source.next() {
                                if word == "end" {
                                    break;
                                }
                                match word {
                                    "print" => {
                                        let name = source.next().unwrap();
                                        writeln!(self.output_stream, "{}", self.variables[name])?;
                                    }
                                    "var" => {
                                        let name = source.next().unwrap();
                                        let value = source.next().unwrap().parse().unwrap();
                                        self.variables.insert(name.to_owned(), value);
                                    }
                                    "add" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() +=
                                            self.variables[name2];
                                    }
                                    _ => {}
                                }
                                parameters.push(word.to_owned());
                            }
                            break;
                        }
                        function.push(word.to_owned());
                    }
                    self.functions.insert(
                        name.to_owned(),
                        Function {
                            parameters,
                            code: function,
                        },
                    );
                }
                "struct" => {
                    let name = source.next().unwrap();
                    let mut struct_fields = HashMap::new();
                    while let Some(field) = source.next() {
                        if field == "endstruct" {
                            break;
                        }
                        let value = source.next().unwrap().parse().unwrap();
                        struct_fields.insert(field.to_owned(), value);
                    }
                    self.structs.insert(name.to_owned(), struct_fields);
                }
                "print" => {
                    let name = source.next().unwrap();
                    match self.variables.get(name) {
                        Some(value) => writeln!(self.output_stream, "{}", value)?,
                        None => match self.arrays.get(name) {
                            Some(array) => {
                                for (index, &value) in array.iter().enumerate() {
                                    writeln!(self.output_stream, "{}[{}] = {}", name, index, value)?;
                                }
                            }
                            None => match self.float.get(name) {
                                Some(value) => writeln!(self.output_stream, "{}", value)?,
                                None => match self.structs.get(name) {
                                    Some(_struct) => {
                                        for (key, value) in _struct.iter() {
                                            writeln!(self.output_stream, "{}.{} = {}", name, key, value)?;
                                        }
                                    }
                                    None => writeln!(self.output_stream, "{}", self.strings[name])?,
                                },
                            },
                        },
                    }
                }
                "call" => {
                    // let name = source.next().unwrap();
                    // let function = &self.functions[name];
                    // let parameters = &function.parameters;
                    // let function = self.functions.get(name).unwrap();
                    // let mut interpreter = Interpreter::new();
                    // for (param_name, param_value) in function.parameters.iter().zip(parameters) {
                    //     interpreter
                    //         .variables
                    //         .insert(param_name.to_owned(), param_value.parse::<i32>().unwrap());
                    // }
                    // interpreter.run(&function.code.join(" "));

                    let name = source.next().unwrap();
                    let function = &self.functions[name];
                    let parameters = &function.parameters;
                    let function_statements = &function.code;
                    let mut local_variables: HashMap<String, i32> = HashMap::new();
                    for (name, parameter) in parameters
                        .iter()
                        .zip(source.by_ref().take(parameters.len()))
                    {
                        local_variables.insert(name.to_owned(), parameter.parse().unwrap());
                    }
                    let mut source = function_statements.iter().cloned().peekable();
                    while let Some(word) = source.next() {
                        match word.as_str() {
                            "var" => {
                                let name = source.next().unwrap();
                                let value = source.next().unwrap().parse().unwrap();
                                local_variables.insert(name, value);
                            }
                            "print" => {
                                let name = source.next().unwrap();
                                writeln!(self.output_stream, "{}", self.variables[&name])?;
                            }
                            "add" => {
                                let name1 = source.next().unwrap();
                                let name2 = source.next().unwrap();
                                *self.variables.get_mut(&name1).unwrap() += self.variables[&name2];
                            }
                            _ => {}
                        }
                    }
                }
                "if" => {
                    let name = source.next().unwrap();
                    let comp = Comparison::from_str(source.next().unwrap());
                    let value = source.next().unwrap().parse().unwrap();
                    let mut executed = false;
                    let condition = match comp {
                        Comparison::Equal => self.variables[name] == value,
                        Comparison::NotEqual => self.variables[name] != value,
                        Comparison::LessThan => self.variables[name] < value,
                        Comparison::LessThanOrEqual => self.variables[name] <= value,
                        Comparison::GreaterThan => self.variables[name] > value,
                        Comparison::GreaterThanOrEqual => self.variables[name] >= value,
                    };
                    if condition {
                        while let Some(word) = source.next() {
                            if word == "else" || word == "end" {
                                break;
                            }
                            match word {
                                "var" => {
                                    let name = source.next().unwrap();
                                    let value = source.next().unwrap().parse().unwrap();
                                    self.variables.insert(name.to_owned(), value);
                                }
                                "end" => {
                                    break;
                                }
                                "print" => {
                                    let name = source.next().unwrap();
                                    writeln!(self.output_stream, "{}", self.variables[name])?;
                                }
                                "add" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() +=
                                        self.variables[name2];
                                }
                                "sub" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() -=
                                        self.variables[name2];
                                }
                                "mul" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() *=
                                        self.variables[name2];
                                }
                                "div" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() /=
                                        self.variables[name2];
                                }
                                _ => {}
                            }
                        }
                    } else {
                        while let Some(word) = source.next() {
                            if word == "end" {
                                break;
                            }
                            if word == "else" {
                                executed = true;
                                while let Some(word) = source.next() {
                                    if word == "end" {
                                        break;
                                    }
                                    match word {
                                        "var" => {
                                            let name = source.next().unwrap();
                                            let value = source.next().unwrap().parse().unwrap();
                                            self.variables.insert(name.to_owned(), value);
                                        }
                                        "print" => {
                                            let name = source.next().unwrap();
                                            writeln!(self.output_stream, "{}", self.variables[name])?;
                                        }
                                        "add" => {
                                            let name1 = source.next().unwrap();
                                            let name2 = source.next().unwrap();
                                            *self.variables.get_mut(name1).unwrap() +=
                                                self.variables[name2];
                                        }
                                        "sub" => {
                                            let name1 = source.next().unwrap();
                                            let name2 = source.next().unwrap();
                                            *self.variables.get_mut(name1).unwrap() -=
                                                self.variables[name2];
                                        }
                                        "mul" => {
                                            let name1 = source.next().unwrap();
                                            let name2 = source.next().unwrap();
                                            *self.variables.get_mut(name1).unwrap() *=
                                                self.variables[name2];
                                        }
                                        "div" => {
                                            let name1 = source.next().unwrap();
                                            let name2 = source.next().unwrap();
                                            *self.variables.get_mut(name1).unwrap() /=
                                                self.variables[name2];
                                        }
                                        "end" => {
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            if executed {
                                break;
                            }
                        }
                    }
                }
                "loop" => {
                    let name = source.next().unwrap();
                    let comp = Comparison::from_str(source.next().unwrap());
                    let value = source.next().unwrap().parse().unwrap();
                    while match comp {
                        Comparison::Equal => self.variables[name] == value,
                        Comparison::NotEqual => self.variables[name] != value,
                        Comparison::LessThan => self.variables[name] < value,
                        Comparison::LessThanOrEqual => self.variables[name] <= value,
                        Comparison::GreaterThan => self.variables[name] > value,
                        Comparison::GreaterThanOrEqual => self.variables[name] >= value,
                    } {
                        let mut inner_source = source.clone();
                        while let Some(word) = inner_source.next() {
                            if word == "end" {
                                break;
                            }
                            match word {
                                "var" => {
                                    let name = inner_source.next().unwrap();
                                    let value = inner_source.next().unwrap().parse().unwrap();
                                    self.variables.insert(name.to_owned(), value);
                                }
                                "print" => {
                                    let name = inner_source.next().unwrap();
                                    writeln!(self.output_stream, "{}", self.variables[name])?;
                                }
                                "add" => {
                                    let name1 = inner_source.next().unwrap();
                                    let name2 = inner_source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() +=
                                        self.variables[name2];
                                }
                                "sub" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() -=
                                        self.variables[name2];
                                }
                                "mul" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() *=
                                        self.variables[name2];
                                }
                                "div" => {
                                    let name1 = source.next().unwrap();
                                    let name2 = source.next().unwrap();
                                    *self.variables.get_mut(name1).unwrap() /=
                                        self.variables[name2];
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "add" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.variables[name1].add(self.variables[name2]);
                    self.variables.insert(name1.to_owned(), result);
                }
                "sub" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.variables[name1].sub(self.variables[name2]);
                    self.variables.insert(name1.to_owned(), result);
                }
                "mul" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.variables[name1].mul(self.variables[name2]);
                    self.variables.insert(name1.to_owned(), result);
                }
                "div" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.variables[name1].div(self.variables[name2]);
                    self.variables.insert(name1.to_owned(), result);
                }
                "add_f" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.float[name1].add(self.float[name2]);
                    self.float.insert(name1.to_owned(), result);
                }
                "sub_f" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.float[name1].sub(self.float[name2]);
                    self.float.insert(name1.to_owned(), result);
                }
                "mul_f" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.float[name1].mul(self.float[name2]);
                    self.float.insert(name1.to_owned(), result);
                }
                "div_f" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.float[name1].div(self.float[name2]);
                    self.float.insert(name1.to_owned(), result);
                }
                "sqrt" => {
                    let name = source.next().unwrap();
                    self.variables
                        .insert(name.to_owned(), (self.variables[name] as f32).sqrt() as i32);
                }
                "abs" => {
                    let name = source.next().unwrap();
                    self.variables
                        .insert(name.to_owned(), self.variables[name].abs());
                }
                "pow" => {
                    let name1 = source.next().unwrap();
                    let name2 = source.next().unwrap();
                    let result = self.variables[name1].pow(self.variables[name2] as u32);
                    self.variables.insert(name1.to_owned(), result);
                }
                "end" => {}
                _ => panic!("Unknown command: {}", word),
            }
        }

        Ok(self.output_stream.by_ref())
    }
    pub fn call_function(&mut self, name: &str, parameters: &[i32]) -> Result<()> {
        let function = self.functions.get(name).unwrap();
        let mut interpreter = Interpreter::new();
        for (param_name, param_value) in function.parameters.iter().zip(parameters) {
            interpreter
                .variables
                .insert(param_name.to_owned(), *param_value);
        }
        interpreter.run(&function.code.join(" "))?;

        Ok(())
    }
}
