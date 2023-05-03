use expression_evaluator::stack::Stack;
use std::io::Write;

fn input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Error");
    s.replace(' ', "").trim().to_string()
}

fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();

    for i in input_chars {
        match i {
            '+' | '-' | '/' | '*' | '^' | '(' | ')' => {
                if temp.len() == 0 {
                    tokenized_input.push(i.to_string())
                } else {
                    tokenized_input.push(temp.into_iter().collect());
                    tokenized_input.push(i.to_string());
                    temp = vec![]
                }
            }
            _ => {
                temp.push(i);
                continue;
            }
        }
    }

    if temp.len() != 0 { tokenized_input.push(temp.into_iter().collect()) }
    tokenized_input
}

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size = input.len();
    let mut stack: Stack<String> = Stack::new(size);
    let mut postfix: Vec<String> = Vec::new();

    for i in input {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                if stack.len() == 0 {
                    stack.push(i);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        stack.push(i);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            postfix.push(stack.pop().unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        stack.push(i);
                    }
                }
            }
            "(" => stack.push(i),
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(stack.pop().unwrap());
                }
                stack.pop().unwrap();
            }
            _ => postfix.push(i),
        }
    }

    while stack.len() != 0 {
        postfix.push(stack.pop().unwrap())
    }

    postfix
}

fn priority(x: &String) -> u8 {
    match x.as_str() {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}

fn operation(op1: String, op2: String, oper: String) -> f32 {
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap();

    match oper.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    }
}

fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size = postfix.len();
    let mut result_stack: Stack<String> = Stack::new(size);

    for i in postfix {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                let oper = i;
                let op2 = result_stack.pop().unwrap();
                let op1 = result_stack.pop().unwrap();
                let result = operation(op1, op2, oper);

                result_stack.push(result.to_string());
            }
            _ => result_stack.push(i),
        }
    }
    result_stack.pop().unwrap().parse::<f32>().unwrap()
}

fn main() {
    let input_expr = input("Enter an expression: ");
    let tokenized_input = individual_symbols(input_expr);
    let postfix = infix_to_postfix(tokenized_input);

    println!("                   = {}", postfix_evaluation(postfix))
}
