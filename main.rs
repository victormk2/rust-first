fn main(){
    let variavel:i16 = 300;
    println!("varivael = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let boolean = false;
    println!("Tamanho boolean = {}", std::mem::size_of_val(&boolean));

    let letra:char = 'C';
    println!("Tamanho do char = {} bytes", std::mem::size_of_val(&letra));
}