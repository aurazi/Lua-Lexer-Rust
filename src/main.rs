mod lexer;
mod token;

// a super basic lua lexer written in Rust

/*
if do then while
repeat until
end else elseif
true false nil in
break continue return
and or not for function
local 
+ - * / ^ %
+= -= *= /= ^= %=
== >= <= < >
[ ] { } #
-5 - 5123
"the world!"
--and so on...
variable0
*/

fn main() {
    let source = r#"
local message_of_doom = "Hello World!"
local function printDoom()
    print(message_of_doom)
end
printDoom()
"#;
    let tokenifier = lexer::Lexer::new(source.chars());
    let tokens = tokenifier.to_tokens();

    println!("{:#?}", tokens);
}