// Debugged with codelldb vscode extension

fn main() {
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
}

fn stack_only(b: i32) -> i32 {
    //breakpoint 
    // b = 2
    // c = 32766
    let c = 3;
    //breakpoint 
    // b = 2
    // c = 3
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    //breakpoint 
    // d = 0
    // e = <null>
    let d = 5;
    let e = Box::new(7);
    //breakpoint 
    // d = 5
    // *e = 7
    return d + *e;
}
