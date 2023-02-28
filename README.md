# Neko programming language

An interpreter for the Neko programming language written in Rust from scratch without using any third-party libraries.

## What’s Neko?

Neko has a Assembly like syntax, supports variable bindings, mathematical operations, has functions and loops, has integers, float numbers, arrays and struct built-in.

## Syntax

* Create a variable

  ```java
  var x 10 
  print x
  ```

* If the else statement

  ```java
  var x 10 
  if x == 10 
    print x
  ```

* Arithmetic operations

  ```java
  var x 30 
  var y 10 
  add x y 
  print x 
  end
  ```

* Loop

  ```java
  var x 0
  var y 1
  loop x < 5
      print x
      add x y
      end
  end
  ```

* Array

  *[keyword == array] [name_of_variable] [size]*

  ```java
  array arr 5 //keyword name_of_variable and size which is 5
    1 2 3 4 5 // elements of array
  print arr
  ```

  Right now array has a static size there is no possibility to make dynamic

* String

  ```java
  string x hello, world endstring
  print x
  ```

* Float

  ```java
  float x 10.34
  print x
  float y 23.15
  print y
  add_f x y
  print x
  ```

* Function

  ```java
  function sum with // void function if after with set name of paramter it will be a function with parameter
    var a 10
    print a
  end
  call sum
  ```

* Struct

  ```java
  struct point 
    x 2 
    y 3 
    z 4 
  endstruct
  print point
  ```

* Switch case

  ```java
  var x 5
  var y 6
  switch x
    case 1
      print y
    case 5
      print x
  ```

## Neko and WASM

ToDo

## ToDo List

❎ Improving functions make it as type based functions and improve setting parameters

❎ Improving struct