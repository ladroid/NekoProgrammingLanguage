use std::collections::HashMap;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Comparison {
    fn from_str(s: &str) -> Self {
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

struct Function {
    parameters: Vec<String>,
    code: Vec<String>,
}

struct Interpreter {
    variables: HashMap<String, i32>,
    arrays: HashMap<String, Vec<i32>>,
    strings: HashMap<String, String>,
    float: HashMap<String, f32>,
    functions: HashMap<String, Function>,
    structs: HashMap<String, HashMap<String, i32>>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            strings: HashMap::new(),
            float: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
        }
    }

    fn run(&mut self, source_code: &str) {
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
                                        println!("{}", self.variables[name]);
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
                        Some(value) => println!("{}", value),
                        None => match self.arrays.get(name) {
                            Some(array) => {
                                for (index, &value) in array.iter().enumerate() {
                                    println!("{}[{}] = {}", name, index, value);
                                }
                            }
                            None => match self.float.get(name) {
                                Some(value) => println!("{}", value),
                                None => match self.structs.get(name) {
                                    Some(_struct) => {
                                        for (key, value) in _struct.iter() {
                                            println!("{}.{} = {}", name, key, value);
                                        }
                                    }
                                    None => println!("{}", self.strings[name]),
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
                                println!("{}", self.variables[&name]);
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
                                    println!("{}", self.variables[name]);
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
                                            println!("{}", self.variables[name]);
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
                                    println!("{}", self.variables[name]);
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
    }
    fn call_function(&mut self, name: &str, parameters: &[i32]) {
        let function = self.functions.get(name).unwrap();
        let mut interpreter = Interpreter::new();
        for (param_name, param_value) in function.parameters.iter().zip(parameters) {
            interpreter
                .variables
                .insert(param_name.to_owned(), *param_value);
        }
        interpreter.run(&function.code.join(" "));
    }
}

fn main() {
    println!("Hello, world!");

    let mut interpreter = Interpreter::new();
    interpreter.run("var x 10 print x");

    let mut interpreter1 = Interpreter::new();
    interpreter1.run("var x 10 if x == 10 print x"); // var x 10 if x 10 var y 20 else var y 30 end print y

    let mut interpreter2 = Interpreter::new();
    interpreter2.run("var x 30 var y 10 add x y print x end");

    let mut interpreter3 = Interpreter::new();
    let source = "var x 30 
                  var y 10 
                  var z 40 
                  add x y 
                  print x
                  add x z
                  print x
                  end";
    interpreter3.run(source);

    let mut interpreter4 = Interpreter::new();
    let source1 = "
        var x 0
        var y 1
        loop x < 5
            print x
            add x y
            end
        end
    ";
    interpreter4.run(source1);

    let mut interpreter5 = Interpreter::new();
    let source2 = "
        var x 10
        var y 5
        var z 1
        if x > 5
          add x y
        end
        loop x < 20
            print x
            add x z
            end
        end
    ";
    interpreter5.run(source2);

    let mut interpreter6 = Interpreter::new();
    let source3 = "
        array arr 5 
          1 2 3 4 5 
        print arr
    ";
    interpreter6.run(source3);
    println!("///////////////////////////");
    let mut interpreter7 = Interpreter::new();
    let source4 = "
        var x 36
        sqrt x
        print x
    ";
    interpreter7.run(source4);
    println!("///////////////////////////");
    let mut interpreter8 = Interpreter::new();
    let source5 = "
        string x hello, world endstring
        print x
    ";
    interpreter8.run(source5);
    println!("///////////////////////////");
    let mut interpreter9 = Interpreter::new();
    let source6 = "
        float x 10.34
        print x
        float y 23.15
        print y
        add_f x y
        print x
    ";
    interpreter9.run(source6);
    println!("///////////////////////////");
    let mut interpreter10 = Interpreter::new();
    let source7 = "
        function sum with x
          var a 10
          print a
        end
        call sum 10
    ";
    interpreter10.run(source7);
    println!("///////////////////////////");
    let mut interpreter11 = Interpreter::new();
    let source8 = "
        struct point 
          x 2 
          y 3 
          z 4 
        endstruct
        print point
    ";
    interpreter11.run(source8);
}
