fn stack_variable(){
    println!("[Stack Variable]");
    let nama_depan = "Pita";
    let nama_belakang = "Pita";
    println!("alamat nama_depan : {:?}",nama_depan.as_ptr());
    println!("alamat nama_belakang : {:?}",nama_belakang.as_ptr());
    println!("apakah alamat sama ? {}",
        if nama_depan.as_ptr() == nama_belakang.as_ptr(){
            "Sama"
        }else{
            "Enga sama"
        }
    )
}

fn heap_variable(){
    println!("[Heap Variable]");
    let nama_depan = String::from("Pita");
    let nama_belakang = String::from("Pita");
    println!("alamat nama_depan : {:?}",nama_depan.as_ptr());
    println!("alamat nama_belakang : {:?}",nama_belakang.as_ptr());
    println!("apakah alamat sama ? {}",
        if nama_depan.as_ptr() == nama_belakang.as_ptr(){
            "Sama"
        }else{
            "Enga sama"
        }
    )
}

fn main(){
    stack_variable();
    println!("");
    heap_variable();
}