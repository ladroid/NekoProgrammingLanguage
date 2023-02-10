use my_project::interpreter::Interpreter;

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
