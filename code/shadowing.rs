fn main (){
    let  x = "new";
    {
        println!("First child scope: {x}");
        let x = 100;
        println!("Second child Scope: {x}");
    }
    println!("Second master scope: {x}");
}