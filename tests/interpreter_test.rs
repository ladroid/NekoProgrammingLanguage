/*
    Tests to validate the output of some source codes
*/

use my_project::interpreter::{Interpreter};

/*
    Utility function to check that the interpreter produces the right output for a given input
*/
fn check_input_output_eq(source_code: &str, expected_output: &str){
    let output_stream: Box<Vec<u8>> = Box::new(Vec::new());
    let mut interpreter = Interpreter::new_with_output_stream(output_stream);
    let output_stream = interpreter.run(source_code).unwrap();
    let actual_ouput = std::str::from_utf8(output_stream.as_slice()).unwrap();
    assert_eq!(expected_output, actual_ouput);
}

/*
    Utility function to check that the interpreter produces at least part of the right output for a given input
*/
fn check_input_output_contains(source_code: &str, expected_part: &str){
    let output_stream: Box<Vec<u8>> = Box::new(Vec::new());
    let mut interpreter = Interpreter::new_with_output_stream(output_stream);
    let output_stream = interpreter.run(source_code).unwrap();
    let actual_ouput = std::str::from_utf8(output_stream.as_slice()).unwrap();
    assert!(actual_ouput.contains(expected_part));
}

#[test]
pub fn test_create_variables(){
    let source_code = "
        var integer_var 5
        float real_var 6.83
        string string_var I am a string endstring
        print integer_var
        print real_var
        print string_var
    ";

    let expected_output = "5\n6.83\nI am a string\n";

    check_input_output_eq(source_code, expected_output)
}

#[test]
pub fn test_add_variables(){
    let source_code = "
        var integer_a 7
        var integer_b 9
        add integer_a integer_b
        print integer_a
        float real_a 13.6
        float real_b 14.9
        add_f real_a real_b
        print real_a
    ";

    let expected_output = "16\n28.5\n";

    check_input_output_eq(source_code, expected_output)
}

#[test]
pub fn test_function_without_parameters(){
    let source_code = "
        function sum_5_6 with
            var a 5
            var b 6
            add a b
            print a
        end
        call sum_5_6
    ";

    let expected_output = "11\n";

    check_input_output_eq(source_code, expected_output)
}

#[test]
pub fn test_struct(){
    let source_code = "
        struct game_scores
            team_a 5
            team_b 3
        endstruct

        print game_scores
    ";

    let expected_output_a = "game_scores.team_a = 5\n";
    let expected_output_b = "game_scores.team_b = 3\n";

    // Members of the struct may be displayed as variables are stored in hash maps, which are unordered
    check_input_output_contains(source_code, expected_output_a);
    check_input_output_contains(source_code, expected_output_b);
}

#[test]
pub fn test_loop(){
    let source_code = "
        var a 15
        var a_add -1
        loop a > 10
            add a a_add
            end
        end
        print a
    ";

    let expected_output = "9\n";

    check_input_output_eq(source_code, expected_output)
}
