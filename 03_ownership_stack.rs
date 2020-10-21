fn main(){
    let nama_depan = "Elisabeth";
    let nama_belakang = "Elisabeth";
    println!("alamat nama_depan : {}",nama_depan.as_ptr());
    println!("alamat nama_belakang : {}",nama_belakang.as_ptr());
    println!("apakah alamat sama ? {}",
        if nama_depan.as_ptr() == nama_belakang.as_ptr(){
            "Sama"
        }else{
            "Enga sama"
        }
    )
}